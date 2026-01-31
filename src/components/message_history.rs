use dioxus::prelude::*;

use crate::packet::ChatMessage;

#[component]
pub fn MessageHistory(messages: Signal<Vec<ChatMessage>>) -> Element {
    rsx! {
        for message in messages.iter() {
            p { "{message.user}: {message.message}" }
        }
    }
}
