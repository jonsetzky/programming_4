use std::time::SystemTime;

use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use uuid::Uuid;

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

// todo move this to tcp chat client's module
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum PacketType {
    None,
    Hello {
        channels_checksum: u32,
        nickname: String,
    },
    KeepAlive,
    Message {
        channel: Uuid,
        message: String,
    },
    RequestChannels {
        known_channels: Vec<Uuid>,
    },
    RespondChannels {
        new_channels: Vec<Channel>,
    },
}

#[derive(Clone, serde::Serialize, Debug, PartialEq)]
pub struct Packet {
    pub id: Uuid,
    pub reply_to: Option<Uuid>,
    pub sender: Uuid,
    pub recipient: Option<Uuid>,
    pub time: DateTime<Utc>,
    pub payload: PacketType,
}

impl Packet {}

// todo add support for modifying user id?
#[derive(Clone, Copy)]
pub struct PacketBuilder {
    user_id: Uuid,
}

impl PacketBuilder {
    pub fn new(user_id: Uuid) -> PacketBuilder {
        PacketBuilder { user_id }
    }

    fn base(self) -> Packet {
        Packet {
            // todo use actual uuids
            id: Uuid::new_v4(),
            // reply_to: Some(Uuid::new_v4()),
            reply_to: None,
            sender: self.user_id,
            time: SystemTime::now().into(),
            payload: PacketType::None,
            recipient: None,
        }
    }

    pub fn chat_message(self, message: String) -> Packet {
        let mut out = self.base();
        out.payload = PacketType::Message {
            message,
            channel: Uuid::new_v4(),
        };
        out
    }
    pub fn keepalive(self) -> Packet {
        let mut out = self.base();
        out.payload = PacketType::KeepAlive;
        out
    }
    pub fn request_channels(self, known_channels: Vec<Uuid>) -> Packet {
        let mut out = self.base();
        out.payload = PacketType::RequestChannels { known_channels };
        out
    }
    pub fn respond_channels(self, new_channels: Vec<Channel>) -> Packet {
        let mut out = self.base();
        out.payload = PacketType::RespondChannels { new_channels };
        out
    }
    pub fn hello(
        self,
        channels_checksum: u32,
        nickname: String,
        recipient: Option<Uuid>,
    ) -> Packet {
        let mut out = self.base();
        out.recipient = recipient;
        out.payload = PacketType::Hello {
            channels_checksum,
            nickname,
        };
        out
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

    fn get_channels(&self) -> Vec<Channel>;
    fn add_channels(&self, channels: Vec<Channel>);
    fn get_channels_checksum(&self) -> u32;
}
