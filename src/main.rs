#[macro_use]
extern crate clap;
extern crate hyper;
extern crate web;
extern crate inverted_index;
extern crate uuid;
extern crate rusqlite;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod rpc;

use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

use clap::App;

use hyper::server::Http;

use inverted_index::manager::Manager;
use inverted_index::manager;
use inverted_index::constants;
use inverted_index::shard;
use inverted_index::manager::StorageEngine;

use rpc::node::Node;
use rpc::messages::Message;
use rpc::db::MetadataDB;

use web::router;
use web::Saga;
use web::handlers::health;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let server_matches = matches.subcommand_matches("server").unwrap();

    let (index_manager_tx, index_manager_rx): (mpsc::Sender<manager::IndexCommand>, mpsc::Receiver<manager::IndexCommand>) = mpsc::channel();
    let index_manager = Manager::new("test_idx", PathBuf::from(constants::TEST_DEFAULT_DATA_DIRECTORY), index_manager_rx, StorageEngine::SQLite, shard::ShardType::Primary);
    
    let my_ip = String::from("127.0.0.1");
    let my_web_port = "3000";
    let my_rpc_port = 3001;
    
    let am_metadata_server: bool;
    if server_matches.is_present("metadata") {
        am_metadata_server = true;
    } else {
        am_metadata_server = false;
    }

    let data_path: &str = server_matches.value_of("data_dir").unwrap_or("/tmp");

    let addr = (my_ip.clone() + ":" + my_web_port).parse().unwrap();

    // Set up the Node struct for this server
    let (my_node_tx, my_node_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let mut my_node = Node::new(my_ip.clone(), my_rpc_port, am_metadata_server, my_node_rx, data_path);
    MetadataDB::create_cluster_table(&mut my_node.db);
    MetadataDB::create_node_table(&mut my_node.db);
    Node::start_rpc_server(my_ip, my_rpc_port as u32, my_node_tx);
    // Starts the RPC listening loop in a background thread
    thread::spawn(move || {
        my_node.receive_message();                    
    });
    // END

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

struct Switchboard {
    index_manager_tx: mpsc::Sender<manager::IndexCommand>,
    my_node_tx: mpsc::Sender<Message>
}