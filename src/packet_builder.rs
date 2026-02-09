use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use uuid::Uuid;

use crate::packet::{ChatMessage, Packet};

#[derive(Debug, Clone)]
pub struct PacketBuilder {
    nickname: Arc<Mutex<String>>,
}

impl PacketBuilder {
    pub fn clone(&self) -> PacketBuilder {
        PacketBuilder {
            nickname: self.nickname.clone(),
        }
    }

    #[allow(unused)]
    pub fn get_nickname(&self) -> String {
        let nick = self.nickname.lock().unwrap();
        nick.to_string()
    }

    pub fn set_nickname(&self, new: &String) {
        let mut nick = self.nickname.lock().unwrap();
        *nick = new.to_string();
    }

    pub fn new(nickname: String) -> PacketBuilder {
        PacketBuilder {
            nickname: Arc::new(Mutex::new(nickname)),
        }
    }

    pub fn chat_message(&self, message: String) -> Packet {
        let timestamp = i64::try_from(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("system time before unix epoch")
                .as_millis(),
        )
        .expect("timestamp wouldn't fit into i64");

        Packet::Chat(ChatMessage {
            id: Uuid::new_v4(),
            inReplyTo: None,
            message,
            user: self.nickname.get_cloned().unwrap(),
            directMessageTo: None,
            sent: timestamp,
        })
    }
    #[allow(unused)]
    pub fn list_channels(&self) -> Packet {
        Packet::ListChannels { channels: None }
    }
    #[allow(unused)]
    pub fn join_channel(&self, channel: String) -> Packet {
        Packet::JoinChannel { channel }
    }
}
