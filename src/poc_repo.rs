use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Duration, Utc};
use crc32fast::Hasher;
use uuid::Uuid;

use crate::repository::{Channel, Message, Repository};

pub struct POCRepo {
    messages: Arc<Mutex<Vec<Message>>>,
    channels: Arc<Mutex<Vec<Channel>>>,
}

impl POCRepo {
    pub fn new() -> POCRepo {
        return POCRepo {
            messages: Arc::new(Mutex::new(vec![Message::new_test("test message")])),
            channels: Arc::new(Mutex::new(vec![Channel::new_test()])),
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
        return self.channels.lock().unwrap().to_vec();
    }
    fn add_channels(&self, channels: Vec<Channel>) {
        let mut channels = channels;
        self.channels.lock().unwrap().append(&mut channels);
    }
    fn get_channels_checksum(&self) -> u32 {
        let mut hasher = Hasher::new();
        for channel in self.get_channels() {
            hasher.update(channel.id.as_bytes());
        }
        hasher.finalize()
    }
    fn get_channels_uuids(&self) -> Vec<Uuid> {
        self.get_channels().iter().map(|c| c.id.clone()).collect()
    }
}
