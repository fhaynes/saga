use std::time;

use serde_json;
use uuid::Uuid;
use std::sync::mpsc;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum MessageType {
    HEARTBEAT,
    REGISTER,
    LIST_NODES,
    SHUTDOWN,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_type: MessageType,
    pub message_id: Uuid,
    pub creation_time: time::SystemTime,
    pub args: Vec<String>,
    
    #[serde(skip_serializing, skip_deserializing)]
    pub response_chan: Option<mpsc::Sender<Message>>
}

impl Message {
    pub fn new(mt: MessageType) -> Message {
        Message{
            message_type: mt,
            message_id: Uuid::new_v4(),
            creation_time: time::SystemTime::now(),
            args: vec![],
            response_chan: None
        }
    }

    pub fn arg(mut self, arg: String) -> Message {
        self.args.push(arg);
        self
    }

    pub fn args(mut self, mut args: Vec<String>) -> Message {
        self.args.append(&mut args);
        self
    }

    pub fn response_chan(mut self, chan: mpsc::Sender<Message>) -> Message {
        self.response_chan = Some(chan);
        self
    }

    pub fn to_vec(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec_pretty(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_message() {
        let msg = Message::new(MessageType::HEARTBEAT).arg("arg1".into()).args(vec!["arg2".into(), "arg3".into()]);
        assert_eq!(msg.message_type, MessageType::HEARTBEAT);
        assert_eq!(msg.args.len(), 3);
    }

    #[test]
    fn test_message_to_vec() {
        let msg = Message::new(MessageType::HEARTBEAT);
        let serialized_msg = msg.to_vec().unwrap();
        assert_eq!(serialized_msg.len(), 200);
    }

}