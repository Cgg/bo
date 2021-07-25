use crate::{Document, Row};

#[test]
fn test_document_get_row() {
    let doc = Document::new(
        vec![Row::from("Hello"), Row::from("world!")],
        "test.rs".to_string(),
    );
    assert_eq!(doc.get_row(0).unwrap().string, "Hello".to_string());
    assert_eq!(doc.get_row(1).unwrap().string, "world!".to_string());
    assert!(doc.get_row(2).is_none());
}

#[test]
fn test_document_is_empty() {
    assert!(Document::new(vec![], "test.rs".to_string(),).is_empty());
    assert!(!Document::new(vec![Row::from("Hello")], "test.rs".to_string()).is_empty());
}

#[test]
fn test_document_num_rows() {
    assert_eq!(Document::new(vec![], "test.rs".to_string()).num_rows(), 0);
    assert_eq!(
        Document::new(vec![Row::from("")], "test.rs".to_string()).num_rows(),
        1
    );
}

#[test]
fn test_document_num_words() {
    assert_eq!(
        Document::new(
            vec![Row::from("Hello world"), Row::from("dear reviewer!")],
            "test.rs".to_string()
        )
        .num_words(),
        4
    );
}

#[test]
fn test_document_row_for_line_number() {
    let row1 = Row::from("Hello world");
    let row2 = Row::from("dear reviewer!");
    assert_eq!(
        Document::new(vec![row1, row2], "test.rs".to_string())
            .row_for_line_number(1)
            .string,
        "Hello world"
    );
}

#[test]
fn test_document_last_line_number() {
    assert_eq!(
        Document::new(
            vec![Row::from("Hello world"), Row::from("dear reviewer!")],
            "test.rs".to_string()
        )
        .last_line_number(),
        2
    );
}
