pub mod queries;

use std::path::Path;

use rusqlite;

use document::Document;
use index::InvertedIndexError;
use store::IndexStore;

pub struct SQLiteStore {
    index_name: String,
    data_path: String,
    connection: rusqlite::Connection,
}

impl SQLiteStore {}

impl IndexStore for SQLiteStore {
    /// Opens an IndexStore or creates a new one
    fn open<S: Into<String>>(name: S, path: &Path) -> Result<Self, rusqlite::Error> {
        match rusqlite::Connection::open(path) {
            Ok(conn) => {
                Ok(SQLiteStore {
                    index_name: name.into(),
                    data_path: String::from(path.to_str().unwrap()),
                    connection: conn,
                })
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    /// Closes an IndexStore, but does not delete it
    fn close(&mut self) -> Result<(), InvertedIndexError> {
        Ok(())
    }

    /// Saves a Document to the IndexStore
    fn save_document(&mut self, document: Document) -> Result<(), InvertedIndexError> {
        Err(InvertedIndexError::new("Not Yet Implemented"))
    }

    /// Deletes a Document from the IndexStore
    fn delete_document_by_id(&mut self, id: u64) -> Result<(), InvertedIndexError> {
        Err(InvertedIndexError::new("Not Yet Implemented"))
    }

    /// Retrieves a Document by id
    fn document_by_id(&mut self, id: u64) -> Result<Document, InvertedIndexError> {
        Err(InvertedIndexError::new("Not Yet Implemented"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_open_sqlite_store() {
        let p = Path::new("test_index.db");
        let c = SQLiteStore::open("test", p);
        assert_eq!(c.is_err(), false);
    }
}
