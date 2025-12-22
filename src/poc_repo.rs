use std::{cell::RefCell, rc::Rc};

use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

use crate::repository::{Message, Repository};

pub struct POCRepo {
    messages: Rc<RefCell<Vec<Message>>>,
}

impl POCRepo {
    pub fn new() -> POCRepo {
        return POCRepo {
            messages: Rc::new(RefCell::new(vec![Message::new_test("test message")])),
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
        return self.messages.borrow().to_vec();
    }
    fn add_message(&self, message: Message) {
        self.messages.borrow_mut().push(message.clone());
    }
    fn get_n_messages_before(
        &self,
        channel_id: Uuid,
        from: DateTime<Utc>,
        count: usize,
    ) -> Vec<Message> {
        return self.messages.borrow().to_vec();
    }
    fn get_unread_message_count(&self, channel_id: Uuid) -> usize {
        return 0;
    }
}
