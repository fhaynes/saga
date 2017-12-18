use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::sync::mpsc;

use rusqlite;

use document::Document;
use constants;
use shard;
use stores::sqlite::queries::*;

#[derive(Clone, Debug)]
/// Enum that represents the various StorageEngines that can be used to store the InvertedIndexes
pub enum StorageEngine {
    /// Uses embedded SQLite databases
    SQLite,
    /// Uses raw files
    Filesystem,
}

/// A Manager instance represents a Shard of an Index on a Node and provides an abstraction around
/// all the functionality required to read/write to the storage backend.
/// 
/// # Example
/// 
/// * Assume we have a cluster of three `Nodes`, A, B and C.
/// * Assume we have an Index with 1 Primary Shard and 1 Replica Shard
/// The Primary Shard might live on `Node` A and the `Replica` `Shard` on Node B. On `Node` A, the
/// `Shard` might have multiple flat files or SQLite DB files, being written to or read from by 
/// multiple threads, but the `Manager` handles coordinating the worker threads and all other resources.
/// 
pub struct Manager {
    /// Name of the `InvertedIndex` this Manager is responsible for
    index_name: String,
    /// `Shard` type that this Manager manages
    shard_type: shard::ShardType,
    /// Base `Path` for where this Manager will keep all its data
    data_directory: PathBuf,
    /// Vector of all the Segments that this Manager manages
    segments: Vec<(mpsc::Sender<IndexCommand>, IndexWorker)>,
    /// Receiver end of the channel by which other resources can send this Manager messages
    receiver: mpsc::Receiver<IndexCommand>,
    /// Number of workers this Manager has, which usually is a 1:1 ratio with threads
    workers: u16,
    /// The backing store for this Manager
    storage_engine: StorageEngine,
}

impl Manager {
    /// Creates and returns a new Manager
    /// 
    /// # Arguments
    /// 
    /// * `name` - Anything that can be converted into a String to be the name of the `InvertedIndex`
    /// * `data_directory` - Root directory that holds all the data for this Manager
    /// * `chan` - Channel by which the Manager can receive commands
    /// * `storage_engine` - The storage engine backing this Manager
    /// * `shard_type` - The type of `Shard` this Manager 
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use std::path::PathBuf;
    /// use inverted_index::{shard,constants};
    /// use inverted_index::manager::*;
    /// use std::sync::mpsc;
    /// let (tx, rx): (mpsc::Sender<IndexCommand>, mpsc::Receiver<IndexCommand>) = mpsc::channel();
    /// let mgr = Manager::new("test_idx", PathBuf::from(constants::TEST_DEFAULT_DATA_DIRECTORY), rx, StorageEngine::SQLite, shard::ShardType::Primary);
    /// ```
    pub fn new<S: Into<String>>(
        name: S,
        data_directory: PathBuf,
        chan: mpsc::Receiver<IndexCommand>,
        storage_engine: StorageEngine,
        shard_type: shard::ShardType,
    ) -> Result<thread::JoinHandle<()>, io::Error> {
        let mut mgr = Manager {
            index_name: name.into(),
            data_directory: data_directory,
            segments: vec![],
            receiver: chan,
            workers: constants::DEFAULT_INDEX_STORE_WORKERS,
            storage_engine: storage_engine.clone(),
            shard_type: shard_type,
        };
        mgr.create_data_directory()?;
        let mut existing_segments = mgr.list_segments()?;
        if existing_segments.len() == 0 {
            mgr.initialize_segments();
            existing_segments = mgr.list_segments()?;
        }
        let join_handle = thread::spawn(move || mgr.run(existing_segments, storage_engine));
        Ok(join_handle)
    }

    fn run(&mut self, existing_segments: Vec<PathBuf>, storage_engine: StorageEngine) {
        for p in existing_segments {
            let (tx, rx): (mpsc::Sender<IndexCommand>, mpsc::Receiver<IndexCommand>) = mpsc::channel();
            let worker = IndexWorker::new(p, rx);
            self.segments.push((tx, worker));
        }

        loop {
            match self.receiver.recv() {
                Ok(msg) => {
                    match msg {
                        IndexCommand::IndexDocument {
                            document,
                            response_channel,
                        } => {
                            println!("Received IndexCommand");
                            match response_channel {
                                Some(ch) => {
                                    match ch.send(true) {
                                        Ok(r) => {
                                            println!("Response sent");
                                        }
                                        Err(e) => {
                                            println!("Error sending response: {}", e);
                                        }
                                    }
                                }
                                None => {}
                            }
                        }
                        IndexCommand::Stats { response_channel } => {
                            match response_channel.send(IndexStats) {
                                Ok(r) => {}
                                Err(e) => {}
                            }
                        }
                        IndexCommand::Ready { response_channel } => {
                            match response_channel.send(true) {
                                Ok(r) => {}
                                Err(e) => {}
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("RecvError: {}", e);
                }
            }
        }
    }

    /// Lists all the segments of a Shard, which translates to just an enumeration of the files
    /// in the directory at the time
    fn list_segments(&self) -> io::Result<Vec<PathBuf>> {
        let mut results = vec![];
        let segment_path: PathBuf = [
            self.data_directory.to_str().unwrap(),
            "indices",
            &self.index_name,
            "segments",
            &self.shard_type.to_string(),
        ].iter()
            .collect();
        for entry in fs::read_dir(segment_path)? {
            let entry = entry?;
            let dir = entry.path();
            results.push(dir);
        }
        Ok(results)
    }

    /// This initializes the needed SQLite databases. NOTE: This should be abstracted more, but is currently
    /// a work in progress.
    fn initialize_segments(&self) {
        for num in 0..self.workers {
            let filename = format!("{}.db", num.to_string());
            let segment_path: PathBuf = [
                self.data_directory.to_str().unwrap(),
                "indices",
                &self.index_name,
                "segments",
                &self.shard_type.to_string(),
                &filename,
            ].iter()
                .collect();
            match rusqlite::Connection::open(segment_path) {
                Ok(conn) => {
                    for query in QUERIES_INITIALIZE_INDEX_DB {
                        match conn.execute(query, &[]) {
                            Ok(_) => {}
                            Err(e) => {
                                println!(
                                    "There was an error executing query: {:?}. Error was: {:?}",
                                    query,
                                    e
                                );
                            }
                        }
                    }
                }
                Err(e) => {}
            }
        }
    }

    /// Convenience function to create the needed data directory
    fn create_data_directory(&self) -> io::Result<()> {
        let segment_path: PathBuf = [
            self.data_directory.to_str().unwrap(),
            "indices",
            &self.index_name,
            "segments",
            &self.shard_type.to_string(),
        ].iter()
            .collect();
        fs::create_dir_all(segment_path)?;
        Ok(())
    }
}

pub struct IndexWorker {
    thread: thread::JoinHandle<()>,
    database_path: PathBuf,
}

impl IndexWorker {
    pub fn new(path: PathBuf, rx: mpsc::Receiver<IndexCommand>) -> IndexWorker {
        let thread = thread::spawn(move || loop {
            let command = rx.recv();
        });
        IndexWorker {
            thread: thread,
            database_path: path,
        }
    }
}

pub enum IndexCommand {
    IndexDocument {
        document: Document,
        response_channel: Option<mpsc::Sender<bool>>,
    },
    Stats { response_channel: mpsc::Sender<IndexStats>, },
    Ready { response_channel: mpsc::Sender<bool>, },
}

pub struct IndexStats;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_create_manager() {
        let (tx, rx): (mpsc::Sender<IndexCommand>, mpsc::Receiver<IndexCommand>) = mpsc::channel();
        match Manager::new(
            "test_idx",
            PathBuf::from(constants::TEST_DEFAULT_DATA_DIRECTORY),
            rx,
            StorageEngine::SQLite,
            shard::ShardType::Primary,
        ) {
            Ok(join_handle) => {
                println!("Join handle is: {:?}", join_handle);
                let (sub_tx, sub_rx): (mpsc::Sender<bool>, mpsc::Receiver<bool>) = mpsc::channel();
                let test_message = IndexCommand::Ready { response_channel: sub_tx };
                tx.send(test_message);
                let response = sub_rx.recv();
                println!("Response received: {:?}", response);
            }
            Err(e) => {
                assert!(false);
            }
        };
    }
}
