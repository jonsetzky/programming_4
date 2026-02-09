use chrono::{DateTime, Local, Timelike};
use dioxus::prelude::*;

use crate::{AppState, packet::ChatMessage};

#[component]
fn Message(message: ChatMessage, is_me: bool) -> Element {
    let time = message.datetime().unwrap();
    let time: DateTime<Local> = time.into();
    let time = format!("{:02}:{:02}", time.hour(), time.minute());

    rsx! {
        div {
            width: "100%",
            text_wrap: "wrap",
            word_wrap: "break-word",
            font_size: "12px",
            overflow_wrap: "break-word",
            justify_items: if is_me { "end" } else { "start" },
            p { margin: "8px 0px 2px 0px", "{message.user}" }
            div {
                max_width: "29rem",
                font_size: "12px",
                background_color: "#262626",
                border_radius: "6px",
                padding: "8px 10px 10px 10px",
                p { user_select: "text", "{message.message}" }
            }
            p { margin: "2px 0px 0px 0px", color: "#727272", "{time}" }
        }
    }
}

#[component]
pub fn MessageHistory(messages: Vec<ChatMessage>) -> Element {
    let state = use_context::<AppState>();
    let username = state.username;

    // todo join continuous messages from same sender during same minute into one block

    rsx! {
        div {
            overflow_y: "scroll",
            display: "flex",
            flex_direction: "column",
            max_width: "36rem",
            height: "100%",
            width: "100%",
            flex_grow: "1",
            justify_content: "flex-start",
            align_items: "center",
            padding: "0px 100px auto 0px",
            for message in messages.iter() {
                Message {
                    message: message.clone(),
                    is_me: message.user == username(),
                }
            }

            // used for autoscrolling when new messages are added
            div { id: "page-anchor", width: "100%", height: "1px" }
        }
    }
}
