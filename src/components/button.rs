use dioxus::{html::style, prelude::*};
use dioxus_logger::tracing;
use regex::Regex;

#[component]
pub fn Button(onclick: Option<Callback>, label: String, class: Option<String>) -> Element {
    rsx! {
        button {
            class,
            r#type: "text",
            onclick: move |evt| {
                onclick.map(|f| f(()));
            },
            "{label}"
        }
    }
}
