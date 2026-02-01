use core::error;
use std::str::FromStr;

use dioxus_desktop::wry::cookie::time::UtcDateTime;
use serde_json::{Value, json};
use serde_with::serde_as;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inReplyTo: Option<Uuid>,
    pub message: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directMessageTo: Option<String>,
    pub sent: i64,
}
impl ChatMessage {
    pub fn datetime(
        &self,
    ) -> Result<UtcDateTime, dioxus_desktop::wry::cookie::time::error::ComponentRange> {
        UtcDateTime::from_unix_timestamp(self.sent)
    }
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[serde_as]
pub enum Packet {
    Error {
        error: String,
        #[serde_as(as = "BoolFromInt")]
        clientshutdown: bool,
    },
    Status {
        status: String,
    },
    Chat(ChatMessage),
    JoinChannel {
        channel: String,
    },
    ChangeTopic {
        topic: String,
    },
    ListChannels {
        #[serde(skip_serializing_if = "Option::is_none")]
        channels: Option<Vec<String>>,
    },
}

impl Packet {
    pub fn into_json(self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(&self)
    }

    pub fn into_bytes(&self) -> Vec<u8> {
        let mut data = serde_json::to_value(&self).expect("coudln't convert packet into json");

        data["type"] = match self {
            Packet::Error { .. } => json!(-1),
            Packet::Status { .. } => json!(0),
            Packet::Chat { .. } => json!(1),
            Packet::JoinChannel { .. } => json!(2),
            Packet::ChangeTopic { .. } => json!(3),
            Packet::ListChannels { .. } => json!(4),
        };

        format!("{data}\n").into_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Packet {
        let val: Value =
            serde_json::from_slice(bytes).expect("Unable to parse incoming data as json");
        let t = val["type"].as_i64().unwrap();
        match t {
            -1 => Packet::Error {
                error: val["error"].as_str().unwrap().into(),
                clientshutdown: val["clientshutdown"].as_i64().unwrap() == 1i64,
            },
            0 => Packet::Status {
                status: val["status"].as_str().unwrap().into(),
            },
            1 => Packet::Chat(ChatMessage {
                id: Uuid::from_str(val["id"].as_str().unwrap())
                    .expect("couldn't deserialize id as UUID"),
                inReplyTo: val["inReplyTo"]
                    .as_str()
                    .map(|r| Uuid::from_str(r).expect("couldn't deserialize id as UUID")),
                message: val["message"].as_str().unwrap().into(),
                user: val["user"].as_str().unwrap().into(),
                directMessageTo: val["directMessageTo"].as_str().map(|s| s.to_string()),
                sent: val["sent"].as_i64().unwrap(),
            }),
            2 => Packet::JoinChannel {
                channel: val["channel"].as_str().unwrap().into(),
            },
            3 => Packet::ChangeTopic {
                topic: val["topic"].as_str().unwrap().into(),
            },
            4 => Packet::ListChannels {
                channels: val["channels"].as_array().map(|arr| {
                    arr.iter()
                        .map(|v| v.as_str().unwrap().to_string())
                        .collect()
                }),
            },
            _ => panic!("unknown Packet type {t}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::time::SystemTime;

    use super::*;

    #[test]
    fn error_deserializes() {
        let val = r#"{
            "type": -1,
            "error" : "Some error message here",
            "clientshutdown": 0
        }"#;
        let packet = Packet::from_bytes(val.as_bytes());
        match packet {
            Packet::Error {
                error,
                clientshutdown,
            } => {
                assert_eq!(error, "Some error message here");
                assert!(!clientshutdown);
            }
            _ => panic!("should be an error packet"),
        }
    }
    #[test]
    fn status_deserializes() {
        let val = r#"{
            "type": 0,
            "status" : "Some status message here"
        }"#;
        let packet = Packet::from_bytes(val.as_bytes());
        match packet {
            Packet::Status { status } => {
                assert_eq!(status, "Some status message here");
            }
            _ => panic!("should be a status packet"),
        }
    }
    #[test]
    fn chat_deserializes_full() {
        let val = r#"{
            "type" : 1,
            "id" : "d4986bec-8026-462a-9a7a-f04eebcf7612",
            "inReplyTo": "d4986bec-8026-462b-8a7a-f04dfbcf1115",
            "message" : "mutta lopettakaa te hurjistelunne ja vierailkaa",
            "user" : "telemakos", 
            "sent":16782697319
        }"#;
        let packet = Packet::from_bytes(val.as_bytes());
        match packet {
            Packet::Chat(chat) => {
                assert_eq!(
                    chat.id,
                    Uuid::parse_str("d4986bec-8026-462a-9a7a-f04eebcf7612").unwrap()
                );
                assert_eq!(
                    chat.inReplyTo,
                    Some(Uuid::parse_str("d4986bec-8026-462b-8a7a-f04dfbcf1115").unwrap())
                );
                assert_eq!(
                    chat.message,
                    "mutta lopettakaa te hurjistelunne ja vierailkaa"
                );
                assert_eq!(chat.user, "telemakos");
                assert_eq!(chat.sent, 16782697319i64);
                assert_eq!(chat.directMessageTo, None);
            }
            _ => panic!("should be a chat packet"),
        }
    }
    #[test]
    fn chat_deserializes_partial() {
        let val = r#"{
            "type" : 1,
            "id" : "d4986bec-8026-462a-9a7a-f04eebcf7612",
            "message" : "mutta lopettakaa te hurjistelunne ja vierailkaa",
            "user" : "telemakos", 
            "sent": 16782697219
        }"#;
        let packet = Packet::from_bytes(val.as_bytes());
        match packet {
            Packet::Chat(chat) => {
                assert_eq!(
                    chat.id,
                    Uuid::parse_str("d4986bec-8026-462a-9a7a-f04eebcf7612").unwrap()
                );
                assert_eq!(chat.inReplyTo, None);
                assert_eq!(
                    chat.message,
                    "mutta lopettakaa te hurjistelunne ja vierailkaa"
                );
                assert_eq!(chat.user, "telemakos");
                assert_eq!(chat.sent, 16782697219);
                assert_eq!(chat.directMessageTo, None);
            }
            _ => panic!("should be a chat packet"),
        }
    }

    #[test]
    fn chat_serializes() {
        let id = Uuid::new_v4();

        let val = Packet::Chat(ChatMessage {
            id,
            inReplyTo: None,
            message: String::from("message"),
            user: String::from("test user"),
            directMessageTo: None,
            sent: 16782697219,
        });

        let parts: Vec<String> = vec![
            format!("\"id\":\"{id}\""),
            String::from("\"message\":\"message\""),
            String::from("\"user\":\"test user\""),
            String::from("\"sent\":16782697219"),
        ];

        let mut ser_val = String::from_utf8(val.into_bytes()).unwrap();

        for part in parts {
            assert_ne!(None, ser_val.find(&part));
            ser_val = ser_val.replace(&part, "");
        }

        let re = Regex::new(r"[\{\},\s]+").unwrap();
        assert!(re.is_match(&ser_val));
    }

    #[test]
    fn chat_from_bytes_reverts_into_bytes() {
        let original = Packet::Chat(ChatMessage {
            id: Uuid::new_v4(),
            inReplyTo: None,
            message: String::from("test message"),
            user: String::from("test user"),
            directMessageTo: None,
            sent: i64::try_from(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )
            .expect("timestamp doesn't fint i64"),
        });

        let bytes = original.into_bytes();
        let str = String::from_utf8(bytes.clone()).unwrap();
        println!("Data: {str}");
        let packet = Packet::from_bytes(&bytes);

        match packet {
            Packet::Chat(p) => match original {
                Packet::Chat(o) => {
                    assert_eq!(p.id, o.id);
                    assert_eq!(p.inReplyTo, o.inReplyTo);
                    assert_eq!(p.message, o.message);
                    assert_eq!(p.user, o.user);
                    assert_eq!(p.sent, o.sent);
                    assert_eq!(p.directMessageTo, o.directMessageTo);
                }
                _ => panic!("should be a chat packet"),
            },
            _ => panic!("should be a chat packet"),
        }
    }
}
