use std::time;

use serde;
use serde_json;

use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    HEARTBEAT,
    REGISTER,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub message_id: Uuid,
    pub creation_time: time::SystemTime,
    pub arguments: Vec<String>
}

