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
    pub fn list_channels(&self) -> Packet {
        Packet::ListChannels { channels: None }
    }
    pub fn join_channel(&self, channel: String) -> Packet {
        Packet::JoinChannel { channel }
    }

    // pub fn from_json(packet: Value) -> Packet {
    //     if data["type"] == MessageType::Chat as i32 {
    //         let mut packet = Packet {
    //             id: Uuid::from_str(
    //                 data["id"]
    //                     .as_str()
    //                     .expect("incoming thread: unable to read packet's id field as str"),
    //             )
    //             .expect("incoming thread: unable to parse uuid of message"),
    //             payload,
    //             reply_to: match data["inReplyTo"].as_str() {
    //                 Some(str) => Some(
    //                     Uuid::from_str(str)
    //                         .expect("incoming thread: unable to parse uuid of message"),
    //                 ),
    //                 None => None,
    //             },
    //             sender: Uuid::from_str(
    //                 String::from(
    //                     data["user"]
    //                         .as_str()
    //                         .expect("incoming thread: unable to read packet's user field as str"),
    //                 )
    //                 .as_str(),
    //             )
    //             .expect("got invalid sender uuid"),
    //             time: DateTime::<Utc>::from_timestamp(
    //                 data["sent"]
    //                     .as_i64()
    //                     .expect("incoming thread: unable to read packet's sent field as u64"),
    //                 0,
    //             )
    //             .expect("incoming thread: unable to parse sent field as datetime"),
    //             recipient: None,
    //         };

    //         if let Some(recipient) = data["directMessageTo"].as_str() {
    //             packet.recipient = Some(
    //                 Uuid::from_str(recipient)
    //                     .expect("incoming thread: unable to parse uuid of message"),
    //             );
    //         }

    //         TcpChatClient::handle_packet(
    //             packet_builder.clone(),
    //             repo_clone.clone(),
    //             tx.clone(),
    //             outgoing_tx.clone(),
    //             other_clients.clone(),
    //             packet,
    //         )
    //         .await;
    //     } else {
    //         println!("incoming (unhandled) data: {}", str);
    //     }
    // }
}
