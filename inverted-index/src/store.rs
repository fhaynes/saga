use rusqlite;

use std::path::Path;

use document::Document;
use index::InvertedIndexError;

/// IndexStore is something that can store and retrieve Documents
pub trait IndexStore: Sized {
    /// Opens an IndexStore or creates a new one
    fn open<S: Into<String>>(name: S, path: &Path) -> Result<Self, rusqlite::Error>;
    /// Closes an IndexStore, but does not delete it
    fn close(&mut self) -> Result<(), InvertedIndexError>;
    /// Saves a Document to the IndexStore
    fn save_document(&mut self, document: Document) -> Result<(), InvertedIndexError>;
    /// Deletes a Document from the IndexStore
    fn delete_document_by_id(&mut self, id: u64) -> Result<(), InvertedIndexError>;
    /// Retrieves a Document by id
    fn document_by_id(&mut self, id: u64) -> Result<Document, InvertedIndexError>;
}