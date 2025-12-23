use std::time::SystemTime;

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use uuid::Uuid;

use crate::tcp_chat_client::{Packet, PacketType};

// pub struct User {
//     id: Uuid,
//     name: String,
// }

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
}

impl Channel {
    pub fn new_test() -> Channel {
        let name = rand::rng()
            .sample_iter(&rand::distr::Alphanumeric)
            .take(4)
            .map(char::from)
            .collect::<String>();
        Channel {
            id: Uuid::new_v4(),
            name: name,
        }
    }
}

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
    pub fn from_packet(packet: &Packet) -> Message {
        match &packet.payload {
            PacketType::Message { channel, message } => {
                Message {
                    id: packet.id,
                    // reply_to: Some(Uuid::new_v4()),
                    sender: packet.sender,
                    time: packet.time,
                    message: message.clone(),
                    channel: channel.clone(),
                }
            }
            _ => panic!("invalid packet type when trying to conver packet to message"),
        }
    }
}

pub trait Repository {
    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn get_unread_message_count(&self, channel_id: Uuid) -> usize;

    fn add_message(&self, message: Message);

    fn get_channels(&self) -> Vec<Channel>;
    fn get_channels_uuids(&self) -> Vec<Uuid>;
    fn add_channels(&self, channels: Vec<Channel>);
    fn get_channels_checksum(&self) -> u32;
}
