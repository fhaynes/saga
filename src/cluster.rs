
use std::sync::mpsc;
use std::thread;
use serde;
use tarpc::sync::{client, server};
use tarpc::sync::client::ClientExt;
use tarpc::util::{FirstSocketAddr, Never};

service! {
    rpc heartbeat(id: String) -> String;
}

#[derive(Clone)]
struct SagaRPCServer;

impl SyncService for SagaRPCServer {
    fn heartbeat(&self, id: String) -> Result<String, Never> {
        Ok(format!("{}", id))
    }
}