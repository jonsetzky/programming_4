use std::time::SystemTime;

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

// todo add easy implementation for converting to json
// pub struct User {
//     id: Uuid,
//     name: String,
// }

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum PacketType {
    KeepAlive,
    Message { message: String },
}
// todo add easy implementation for converting to json
#[derive(Clone, serde::Serialize, Debug, PartialEq)]
pub struct Packet {
    pub id: Uuid,
    pub reply_to: Option<Uuid>,
    pub sender: String,
    pub time: DateTime<Utc>,
    pub payload: PacketType,
}

impl Packet {
    pub fn chat_message(message: String) -> Packet {
        Packet {
            id: Uuid::new_v4(),
            reply_to: Some(Uuid::new_v4()),
            sender: String::from("test sender"),
            time: SystemTime::now().into(),
            payload: PacketType::Message { message },
        }
    }
    pub fn keepalive() -> Packet {
        Packet {
            id: Uuid::new_v4(),
            reply_to: Some(Uuid::new_v4()),
            sender: String::from("test sender"),
            time: SystemTime::now().into(),
            payload: PacketType::KeepAlive,
        }
    }
}

// pub struct Channel {
//     id: Uuid,
//     name: String,
// }

// todo add easy implementation for converting to json
#[derive(Clone, serde::Serialize, Debug, PartialEq)]
pub struct Message {
    pub id: Uuid,
    pub channel: Uuid,
    pub sender: Uuid,
    // reply_to: Uuid,
    pub time: DateTime<Utc>,
    pub message: String,
}

impl Message {
    pub fn new_test(message: &str) -> Message {
        Message {
            id: Uuid::new_v4(),
            // reply_to: Some(Uuid::new_v4()),
            sender: Uuid::new_v4(),
            time: SystemTime::now().into(),
            message: String::from(message),
            channel: Uuid::new_v4(),
        }
    }
}

pub trait Repository {
    fn get_message_range(
        &self,
        channel_id: Uuid,
        to: DateTime<Utc>,
        since: Duration,
    ) -> Vec<Message>;

    fn get_n_messages_before(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        count: usize,
    ) -> Vec<Message>;

    fn get_unread_message_count(&self, channel_id: Uuid) -> usize;

    fn add_message(&self, message: Message);
}
