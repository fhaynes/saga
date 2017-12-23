#![feature(plugin, use_extern_macros)]
#![plugin(tarpc_plugins)]
#[macro_use]
extern crate tarpc;
extern crate clap;
extern crate hyper;
extern crate web;
extern crate inverted_index;
extern crate serde;

mod cluster;

use std::io;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::process::exit;

use clap::App;

use tarpc::sync::{client, server};
use tarpc::sync::client::ClientExt;
use tarpc::util::{FirstSocketAddr, Never};

use hyper::server::Http;

use inverted_index::manager::Manager;
use inverted_index::manager;
use inverted_index::constants;
use inverted_index::shard;
use inverted_index::manager::StorageEngine;

use web::router;
use web::Saga;
use web::handlers::health;

fn main() {
    let (tx, rx): (mpsc::Sender<manager::IndexCommand>, mpsc::Receiver<manager::IndexCommand>) = mpsc::channel();
    let index_manager = Manager::new("test_idx", PathBuf::from(constants::TEST_DEFAULT_DATA_DIRECTORY), rx, StorageEngine::SQLite, shard::ShardType::Primary);
    
    let addr = "127.0.0.1:3000".parse().unwrap();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let (atx, arx) = mpsc::channel();
    thread::spawn(move || {
        let mut handle = cluster::SagaRPCServer.listen("localhost:4000", server::Options::default())
            .unwrap();
        atx.send(handle.addr()).unwrap();
        handle.run();
    });

    let server = Http::new().bind(&addr, || {
        let mut router = router::Router::new();
        let health_route = router::Route::new("/healthz", hyper::Method::Get, health::health_check).unwrap();
        router.add_route(health_route);
        let saga = Saga{
            router: router
        };
        Ok(saga)
    }).unwrap();
    server.run().unwrap();
}
