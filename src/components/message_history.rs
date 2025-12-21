use dioxus::prelude::*;

use crate::message_repository::Message;

#[component]
pub fn MessageHistory(messages: Signal<Vec<Message>>) -> Element {
    rsx! {
        for message in messages.iter() {
            p { "{message.sender}: {message.message}" }
        }
    }
}
