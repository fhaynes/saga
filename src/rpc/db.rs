use rusqlite::Connection;

/// Query to create the Cluster table
pub const QUERY_CREATE_CLUSTER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS cluster (
        id              TEXT PRIMARY KEY NOT NULL,
        name            TEXT
    )";

pub const QUERY_CREATE_NODE_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS nodes (
        id              TEXT PRIMARY KEY NOT NULL,
        name            TEXT
    )";

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
}