use std::error::Error;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

/// Represents a discrete collection of text that we want to index
pub struct Document {
    /// Unique numerical identifier for the document
    id: Option<u64>,
    /// The raw text of the Document
    raw: String,
    /// HashMap that stores a term and a vector of all the locations it is found at in the document
    locations: HashMap<String, Vec<u64>>,
}

impl Document {
    /// Returns a Document with the given ID and raw content
    ///
    /// # Arguments
    ///
    /// * `id` - A number that uniquely identifies this Document within the index
    /// * `raw` - This is the raw content as supplied by the client
    ///
    /// # Example
    ///
    /// ```
    /// use inverted_index::document::Document;
    /// let document = Document::new(0, "This is a test");
    /// ```
    pub fn new(id: u64, raw: &str) -> Document {
        let mut document = Document {
            id: Some(id),
            raw: raw.to_owned(),
            locations: HashMap::new(),
        };
        Document::process(&mut document);
        document
    }

    /// Sets the raw field of a Document. Meant to be used as part of the Builder pattern.
    ///
    /// # Arguments
    ///
    /// * `r` - A String that is the raw content to add to the Document
    ///
    /// # Example
    ///
    /// ```
    /// use inverted_index::document::Document;
    /// let document = Document::new(0, "").raw("This is a test");
    /// ```
    pub fn raw(mut self, r: String) -> Document {
        self.raw = r;
        self
    }

    /// Splits the raw field of a Document into Terms
    ///
    /// # Arguments
    ///
    /// * `doc` - Mutable reference to the Document we want to process
    ///
    /// # Example
    ///
    /// ```
    /// use inverted_index::document::Document;
    /// let document = Document::new(0, "This is a test");
    /// Document::process(&mut document);
    /// ```
    fn process(doc: &mut Document) {
        let results = split_on_whitespace(&doc.raw);
        for (term, offset) in results {
            if !doc.locations.contains_key(&term) {
                doc.locations.insert(term.clone(), vec![]);
            }
            match doc.locations.get_mut(&term) {
                Some(v) => v.push(offset),
                None => {}
            }
        }
    }
}


impl FromStr for Document {
    /// Implements FromStr for Document so that we can easily turn a `str` into a `Document`
    ///
    /// # Arguments
    ///
    /// * `s` - &str that we want to put into a `Document`
    ///
    /// # Failures
    ///
    /// There should currently never be a circumstance where DocumentError is returned, but it
    /// may be the case in the future.
    ///
    /// # Example
    ///
    /// ```
    /// use inverted_index::document::Document;
    /// let document = Document::from_str("This is a test");
    /// ```
    type Err = DocumentError;
    fn from_str(s: &str) -> Result<Document, Self::Err> {
        Ok(Document {
            id: None,
            raw: s.to_owned(),
            locations: HashMap::new(),
        })
    }
}

impl fmt::Display for DocumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
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
    ///
    /// # Arguments
    ///
    /// * `msg` - The error message we want to include in the DocumentError
    ///
    /// # Example
    ///
    /// ```
    /// use inverted_index::document::Document;
    /// let document_error = DocumentError::new("Invalid character in Document!");
    /// ```
    fn new(msg: &str) -> DocumentError {
        DocumentError { details: msg.to_string() }
    }
}

/// Splits the value of a document on whitespace and returns a vector of tuples
/// containing terms and offsets
///
/// # Arguments
///
/// * `value` - The line of text we want to split
///
/// # Example
///
/// ```
/// let terms = split_on_whitespace("This is a test");
/// ```
///
/// The result Vector would be: [("This", 0), ("is", 1), ("a", 2), ("test", 3)]
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

}
