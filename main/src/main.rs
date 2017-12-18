extern crate hyper;
extern crate web;
extern crate inverted_index;

use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::process::exit;

use hyper::server::Http;

use inverted_index::manager::Manager;
use inverted_index::manager;
use inverted_index::constants;
use inverted_index::shard;
use inverted_index::manager::StorageEngine;

use web::Saga;

fn main() {
    let (tx, rx): (mpsc::Sender<manager::IndexCommand>, mpsc::Receiver<manager::IndexCommand>) = mpsc::channel();
    let index_manager = Manager::new("test_idx", PathBuf::from(constants::TEST_DEFAULT_DATA_DIRECTORY), rx, StorageEngine::SQLite, shard::ShardType::Primary);
    
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Saga)).unwrap();
    server.run().unwrap();
}
