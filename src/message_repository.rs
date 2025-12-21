use bounded_integer::BoundedUsize;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

// todo add easy implementation for converting to json
pub struct User {
    id: Uuid,
    name: String,
}

// todo add easy implementation for converting to json
#[derive(Clone)]
pub struct Message {
    pub id: Uuid,
    pub reply_to: Option<Uuid>,
    pub sender: String,
    pub time: DateTime<Utc>,
    pub message: String,
}

pub struct Channel {
    id: Uuid,
    name: String,
}

pub trait MessageRepository {
    fn get_message_range(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        to: Duration,
    ) -> Vec<Message>;

    fn get_n_messages_before<const N: usize>(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        count: BoundedUsize<1, 50>,
    ) -> Vec<Message>;

    fn get_unread_message_count(&self, channel_id: Uuid) -> usize;

    fn add_message(&mut self, message: &Message);
}
