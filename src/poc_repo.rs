use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::repository::{Message, Repository};

pub struct POCRepo {
    messages: Arc<Mutex<Vec<Message>>>,
}

impl POCRepo {
    pub fn new() -> POCRepo {
        return POCRepo {
            messages: Arc::new(Mutex::new(vec![Message::new_test("test message")])),
        };
    }
}

impl Repository for POCRepo {
    fn get_message_range(
        &self,
        channel_id: Uuid,
        to: DateTime<Utc>,
        since: Duration,
    ) -> Vec<Message> {
        return self.messages.lock().unwrap().to_vec();
    }
    fn add_message(&self, message: Message) {
        self.messages.lock().unwrap().push(message.clone());
    }
    fn get_n_messages_before(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        count: usize,
    ) -> Vec<Message> {
        return self.messages.lock().unwrap().to_vec();
    }
    fn get_unread_message_count(&self, channel_id: Uuid) -> usize {
        return 0;
    }
    fn get_channels(&self) -> Vec<crate::repository::Channel> {
        return vec![];
    }
}
