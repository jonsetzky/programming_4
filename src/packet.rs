use dioxus_desktop::wry::cookie::time::UtcDateTime;
use serde_json::Value;
use uuid::Uuid;

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub id: Uuid,
    pub inReplyTo: Option<Uuid>,
    pub message: String,
    pub user: String,
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
#[serde(tag = "type")]
pub enum Packet {
    #[serde(rename = "-1")]
    Error { error: String, clientshutdown: bool },
    #[serde(rename = "0")]
    Status { status: String },
    #[serde(rename = "1")]
    Chat(ChatMessage),
    #[serde(rename = "2")]
    JoinChannel { channel: String },
    #[serde(rename = "3")]
    ChangeTopic { topic: String },
    #[serde(rename = "4")]
    ListChannels,
}

impl Packet {
    pub fn into_json(self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_deserializes() {}
    #[test]
    fn status_deserializes() {}
    #[test]
    fn chat_deserializes() {}
    #[test]
    fn join_channel_deserializes() {}
    #[test]
    fn change_topic_deserializes() {}
    #[test]
    fn list_channels_deserializes() {}

    #[test]
    fn chat_serializes() {}
    #[test]
    fn join_channel_serializes() {}
    #[test]
    fn change_topic_serializes() {}
}
