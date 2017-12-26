#[macro_use]
extern crate clap;
extern crate uuid;
extern crate rusqlite;
extern crate serde;
extern crate serde_json;

extern crate hyper;
extern crate web;
extern crate inverted_index;
extern crate rpc;

use std::path::PathBuf;
use std::time;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

use clap::App;
use hyper::server::Http;

use inverted_index::manager::Manager;
use inverted_index::manager;
use inverted_index::shard;
use inverted_index::manager::StorageEngine;

use rpc::node::{Node, NodeConfiguration};
use rpc::messages::Message;
use rpc::db::MetadataDB;

use web::router;
use web::{Saga, ServiceConfiguration};
use web::handlers::health;
use web::handlers::cluster;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let server_matches = matches.subcommand_matches("server");
    let server_matches = server_matches.unwrap();

    let data_path = server_matches.value_of("data_path").unwrap_or("/tmp/saga/");

    let (_index_manager_tx, index_manager_rx): (mpsc::Sender<manager::IndexCommand>, mpsc::Receiver<manager::IndexCommand>) = mpsc::channel();
    let _index_manager = Manager::new("test_idx", PathBuf::from(data_path), index_manager_rx, StorageEngine::SQLite, shard::ShardType::Primary);
    
    let metadata_address: &str;
    let metadata_port: &str;

    let web_address: &str;
    let web_port: &str;

    let rpc_address: &str;
    let rpc_port: &str;
    
    let am_metadata_server: bool;
    if server_matches.is_present("metadata") {
        am_metadata_server = true;
    } else {
        am_metadata_server = false;
    }

    let temp_name = uuid::Uuid::new_v4().to_string();

    let node_name = server_matches.value_of("name").unwrap_or(&temp_name);
    metadata_address = server_matches.value_of("metadata_address").unwrap_or("127.0.0.1");
    metadata_port = server_matches.value_of("metadata_port").unwrap_or("3000");
    rpc_address = server_matches.value_of("rpc_address").unwrap_or("127.0.0.1");
    rpc_port = server_matches.value_of("rpc_port").unwrap_or("3001");
    web_address = server_matches.value_of("web_address").unwrap_or("127.0.0.1");
    web_port = server_matches.value_of("web_port").unwrap_or("2999");

    let data_path: &str = server_matches.value_of("data_dir").unwrap_or("/tmp");

    let addr = (web_address.to_owned() + ":" + web_port).parse().unwrap();

    // Set up the Node struct for this server
    // TODO: This should be broken out into a function somewhere
    let (my_node_tx, my_node_rx): (mpsc::Sender<Message>, mpsc::Receiver<Message>) = mpsc::channel();
    let my_node_config = NodeConfiguration {
        name: node_name.to_owned(),
        metadata_address: metadata_address.into(),
        metadata_port: metadata_port.parse::<u16>().unwrap(),
        data_path: data_path.to_owned(),
        am_metadata_server: am_metadata_server,
        rx: Arc::new(Mutex::new(my_node_rx)),
        rpc_address: rpc_address.to_owned(),
        rpc_port: rpc_port.parse::<u16>().unwrap()
    };

    let mut my_node = Node::new(my_node_config);
    MetadataDB::create_cluster_table(&mut my_node.db);
    MetadataDB::create_node_table(&mut my_node.db);

    // Set up the RPC server and start it
    // TODO: There may be a cleaner way to handle this without so many clones
    let cloned_rpc_address = rpc_address.to_owned();
    let cloned_rpc_port = rpc_port.parse::<u32>().unwrap().clone();
    let my_node_tx = Arc::new(Mutex::new(my_node_tx));
    let cloned_my_node_tx = my_node_tx.clone();
    thread::spawn(move || {
        Node::start_rpc_server(cloned_rpc_address, cloned_rpc_port, cloned_my_node_tx.clone());
    });
    
    // If we aren't the metadata server, we need to establish a connection and register with the
    // metadata server
    if !am_metadata_server {
        loop {
            if my_node.register_with_metadata_server().is_err() {
                println!("There was an error registering with the metadata server! Will sleep 5 seconds and retry");
                thread::sleep(time::Duration::from_millis(5000));
            } else {
                break;
            }
        }
    }

    // Starts the RPC listening loop in a background thread
    thread::spawn(move || {
        my_node.receive_message();                    
    });
    // END

    let swb = 
        Arc::new(
            Mutex::new(
                rpc::Switchboard::new(my_node_tx.clone())
            )
        );
    
    // Configure and start up the web server
    
    let cloned_data_path = data_path.to_owned();
    let server = Http::new().bind(&addr, move || {
        let mut router = router::Router::new();
        let service_config = ServiceConfiguration::new(cloned_data_path.to_owned(), swb.clone());

        let health_route = router::Route::new("/healthz", hyper::Method::Get, health::health_check).unwrap();
        router.add_route(health_route);

        let node_list_route = router::Route::new("/nodes", hyper::Method::Get, cluster::list_nodes).unwrap();
        router.add_route(node_list_route);

        let saga = Saga{
            router: router,
            config: service_config
        };
        Ok(saga)
    }).unwrap();
    println!("Starting web server on {}:{}", web_address, web_port);
    server.run().unwrap();
}