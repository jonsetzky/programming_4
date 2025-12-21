use std::time::SystemTime;

use bounded_integer::BoundedUsize;
use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::message_repository::{Message, MessageRepository};

struct POCRepo {
    messages: Vec<Message>,
}

impl POCRepo {
    fn new() -> POCRepo {
        return POCRepo {
            messages: vec![Message {
                id: Uuid::new_v4(),
                reply_to: Uuid::new_v4(),
                sender: String::from("test sender"),
                time: SystemTime::now().into(),
                message: String::from("test message"),
            }],
        };
    }
}

#[allow(unused_variables)]
impl MessageRepository for POCRepo {
    fn get_message_range(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        to: Duration,
    ) -> Vec<Message> {
        return self.messages.to_vec();
    }
    fn add_message(&mut self, message: &Message) {
        self.messages.push(message.clone());
    }
    fn get_n_messages_before<const N: usize>(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        count: BoundedUsize<1, 50>,
    ) -> Vec<Message> {
        return self.messages.to_vec();
    }
    fn get_unread_message_count(&self, channel_id: Uuid) -> usize {
        return 0;
    }
}
