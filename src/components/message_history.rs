use dioxus::prelude::*;

#[component]
pub fn MessageHistory(messages: Signal<Vec<String>>) -> Element {
    rsx! {
        for message in messages.iter() {
            p { "{message}" }
        }
    }
}
