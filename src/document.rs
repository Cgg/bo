use crate::Row;
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::io::{Error, Write};
use std::path;
use std::slice::{Iter, IterMut};

pub struct Document {
    rows: Vec<Row>,
    pub filename: String,
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(self.filename.as_str()).finish()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            rows: vec![Row::from("")],
            filename: "".to_string(),
        }
    }
}

impl Document {
    #[must_use]
    pub fn new(rows: Vec<Row>, filename: String) -> Self {
        Self { rows, filename }
    }

    #[must_use]
    pub fn new_empty(filename: String) -> Self {
        Self {
            rows: vec![Row::from("")],
            filename,
        }
    }

    /// # Panics
    ///
    /// This function will panic if the path contains a non UTF-8 character
    #[must_use]
    pub fn swap_filename(filename: &str) -> String {
        let filepath = path::Path::new(filename);
        let parent = filepath.parent().unwrap();
        let stripped_filename = filepath.file_name().unwrap();
        let new_filename = format!(".{}.swp", stripped_filename.to_str().unwrap());
        let joined_os_str = parent.join(new_filename);
        let out = joined_os_str.as_os_str().to_str().unwrap_or_default();
        String::from(out)
    }

    /// # Errors
    ///
    /// Returns an error if a file bearing the provided filename
    /// cannot be open.
    pub fn open(filename: &str) -> Result<Self, Error> {
        if !path::Path::new(filename).is_file() {
            return Ok(Self::new_empty(String::from(filename)));
        }
        let file_contents = if path::Path::new(&Self::swap_filename(filename)).is_file() {
            fs::read_to_string(Self::swap_filename(filename))?
        } else {
            fs::read_to_string(filename)?
        };

        let mut rows = Vec::new();
        for line in file_contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self {
            rows,
            filename: filename.to_string(),
        })
    }

    /// # Errors
    ///
    /// Can return an error if the file can't be created or written to.
    pub fn save_to_swap_file(&self) -> Result<(), Error> {
        if !Self::swap_filename(self.filename.as_str()).is_empty() {
            let mut file = fs::File::create(Self::swap_filename(self.filename.as_str()))?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
        }
        Ok(())
    }

    pub fn trim_trailing_spaces(&mut self) {
        for row in self.iter_mut() {
            row.trim_end_inplace();
        }
    }

    /// # Errors
    ///
    /// Can return an error if the file can't be created or written to.
    pub fn save(&self) -> Result<(), Error> {
        if !self.filename.is_empty() {
            let mut file = fs::File::create(self.filename.as_str())?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            if fs::remove_file(Self::swap_filename(self.filename.as_str())).is_ok() {
                // pass
            }
        }
        Ok(())
    }

    #[must_use]
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.len() == 0
    }

    #[must_use]
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    #[must_use]
    pub fn num_words(&self) -> usize {
        self.iter().map(Row::num_words).sum()
    }

    /// Get the document row corresponding to a given line number
    #[must_use]
    pub fn row_for_line_number(&self, line_number: usize) -> Option<&Row> {
        self.get_row(line_number.saturating_sub(1))
    }

    /// Return the line number of the last line in the file
    #[must_use]
    pub fn last_line_number(&self) -> usize {
        self.num_rows()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<Row> {
        self.rows.iter()
    }

    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<Row> {
        self.rows.iter_mut()
    }

    pub fn insert(&mut self, c: char, x: usize, y: usize) {
        match y.cmp(&self.num_rows()) {
            Ordering::Equal | Ordering::Greater => {
                let mut row = Row::default();
                row.insert(0, c);
                self.rows.push(row);
            }
            Ordering::Less => {
                if let Some(row) = self.rows.get_mut(y) {
                    row.insert(x, c);
                }
            }
        }
    }

    pub fn delete(&mut self, x: usize, from_x: usize, y: usize) {
        if y >= self.num_rows() {
            return;
        }
        if let Some(row) = self.rows.get_mut(y) {
            // Deletion at the very start of a line means we append the current line to the previous one
            if x == 0 && from_x == 0 && y > 0 {
                let current_row = self.rows.remove(y);
                if let Some(previous_row) = self.rows.get_mut(y - 1) {
                    previous_row.append(&current_row);
                }
            } else {
                row.delete(x);
            }
        }
    }

    pub fn insert_newline(&mut self, x: usize, y: usize) {
        if y > self.num_rows() {
            return;
        }
        let current_row = self.rows.get_mut(y);
        if let Some(current_row) = current_row {
            if x < current_row.len().saturating_sub(1) {
                let split_row = current_row.split(x);
                self.rows.insert(y.saturating_add(1), split_row);
                // newline inserted in the middle of the row
            } else {
                let new_row = Row::default();
                if y == self.num_rows() || y.saturating_add(1) == self.num_rows() {
                    self.rows.push(new_row);
                } else {
                    self.rows.insert(y.saturating_add(1), new_row);
                }
            }
        }
    }

    pub fn delete_row(&mut self, y: usize) {
        if y > self.num_rows() {
        } else if self.num_rows() == 1 {
            if let Some(row) = self.rows.get_mut(0) {
                row.string = "".to_string();
            }
        } else if self.rows.get(y).is_some() {
            self.rows.remove(y);
        }
    }
}

#[cfg(test)]
#[path = "./document_test.rs"]
mod document_test;
