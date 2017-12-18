use std::error::Error;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fmt;
use std::str::FromStr;

/// Represents a collection of text that was indexes
pub struct Document {
    id: Option<u64>,
    raw: String,
    locations: HashMap<String, Vec<u64>>
}

impl<'a> Document {
    /// Creates and returns a new Document
    pub fn new(id: u64, raw: &str) -> Document {
        let mut document = Document {
            id: Some(id),
            raw: raw.to_owned(),
            locations: HashMap::new()
        };
        Document::process(&mut document);
        document
    }

    /// Builder pattern function to set the raw of a Document
    pub fn raw(mut self, r: String) -> Document {
        self.raw = r;
        self
    }

    fn process(doc: &mut Document) {
        let results = split_on_whitespace(&doc.raw);
        for (term, offset) in results {
            if !doc.locations.contains_key(&term) {
                doc.locations.insert(term.clone(), vec![]);
            }
            match doc.locations.get_mut(&term) {
                Some(v) => {
                    v.push(offset)
                },
                None => {}
            }
        }
    }
}

impl<'a> FromStr for Document {
    type Err = DocumentError;
    fn from_str(s: &str) -> Result<Document, Self::Err> {
        Ok(Document {
            id: None,
            raw: s.to_owned(),
            locations: HashMap::new()
        })
    }
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for DocumentError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct DocumentError {
    details: String,
}

impl DocumentError {
    /// Creates and returns a new DocumentError
    fn new(msg: &str) -> DocumentError {
        DocumentError{
            details: msg.to_string()
        }
    }
}

/// Splits the value of a document on whitespace and returns a vector of terms and offsets
fn split_on_whitespace(value: &str) -> Vec<(String, u64)> {
    let mut result: Vec<(String, u64)> = Vec::new();
    for (i, term) in value.split_whitespace().enumerate() {
        result.push((term.to_owned(), i as u64));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;

    #[test]
    fn test_create_document() {
        let new_document = Document::new(0, "Test");
        assert_eq!(new_document.id.unwrap(), 0);
    }

    #[test]
    fn test_document_from_string() {
        let new_document = Document::from_str("Test").unwrap();
        assert_eq!(new_document.id, None);
        assert_eq!(new_document.raw, "Test");
    }

    #[test]
    fn test_split_document() {
        let new_document = Document::from_str("This is a test").unwrap();
        let results = split_on_whitespace(&new_document.raw);
        assert_eq!(results.len(), 4);
    }

    // #[bench]
    // fn bench_split_document(b: &mut Bencher) {
    //     b.iter(|| {
    //             let new_document = Document::from_str("This is a test").unwrap();
    //             let results = split_on_whitespace(&new_document.raw);
    //         }
    //     )
    // }

}