use rusqlite::Connection;

/// Query to create the Cluster table
pub const QUERY_CREATE_CLUSTER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS cluster (
        id              TEXT PRIMARY KEY NOT NULL,
        name            TEXT
    )";

pub const QUERY_CREATE_NODE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS nodes (
        name            TEXT PRIMARY KEY NOT NULL,
        host            TEXT NOT NULL,
        port            INT NOT NULL
    )";

pub const QUERY_LIST_NODES: &'static str = "SELECT name FROM nodes";

pub const QUERY_REGISTER_NODE: &'static str = "INSERT OR IGNORE INTO nodes VALUES (?, ?, ?)";

pub struct MetadataDB;

impl MetadataDB {
    pub fn create_cluster_table(conn: &Connection) {
        match conn.execute(QUERY_CREATE_CLUSTER_TABLE, &[]) {
            Ok(c) => {
                println!("Cluster table created");
            },
            Err(e) => {
                println!("There was an error creating the cluster table: {:?}", e);
            },
        }
    }

    pub fn create_node_table(conn: &Connection) {
        match conn.execute(QUERY_CREATE_NODE_TABLE, &[]) {
            Ok(c) => {
                println!("Node table created!");
            },
            Err(e) => {
                println!("There was an error creating the node table: {:?}", e);
            },
        }
    }

    pub fn register_node(conn: &Connection, name: &str, hostname: &str, port: u16) -> bool {
        match conn.execute(QUERY_REGISTER_NODE, &[&name, &hostname, &port]) {
            Ok(c) => {
                true
            },
            Err(e) => {
                println!("There was an error registering a node: {}", e);
                false
            }
        }
    }

    pub fn list_nodes(conn: &Connection) -> Vec<String> {
        let mut results = vec![];
        let mut stmt = conn.prepare(QUERY_LIST_NODES).unwrap();

        let node_iter = stmt.query_map(&[], |row| {
            row.get(0)
        }).unwrap();

        for node in node_iter {
            results.push(node.unwrap());
        }
        results

    }
}