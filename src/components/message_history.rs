use chrono::{DateTime, Datelike, Local, Timelike};
use dioxus::prelude::*;

use crate::{AppState, packet::ChatMessage};

#[component]
fn Message(message: UIChatMessage, is_me: bool) -> Element {
    let time = message.message.datetime().unwrap();
    let time: DateTime<Local> = time.into();
    let time = format!("{:02}:{:02}", time.hour(), time.minute());

    let content = message.message.message;
    let user = message.message.user;
    let show_user = message.show_user;
    let show_time = message.show_time;

    rsx! {
        div {
            width: "100%",
            text_wrap: "wrap",
            word_wrap: "break-word",
            font_size: "12px",
            overflow_wrap: "break-word",
            justify_items: if is_me { "end" } else { "start" },
            if show_user {
                p { margin: "8px 0px 2px 0px", "{user}" }
            }
            div {
                max_width: "29rem",
                font_size: "12px",
                background_color: "#262626",
                border_radius: "6px",
                padding: "8px 10px 10px 10px",
                p { user_select: "text", "{content}" }
            }
            if show_time {
                p { margin: "2px 0px 0px 0px", color: "#727272", "{time}" }
            }
        }
    }
}

fn scroll_to_anchor() {
    document::eval(r#"document.getElementById("page-anchor").scrollIntoView()"#);
}

// right after a message is sent, the distance is 66
const AUTOSCROLL_THRESHOLD: f64 = 100.0;

async fn should_autoscroll() -> Option<bool> {
    // returns array where [current scroll, max scroll]
    // "let c = document.getElementById("message-history-container");return [c.scrollTop, c.scrollHeight - c.offsetHeight]"

    // returns distance to max scroll (distance to bottom)
    let eval = match document::eval(
        r#"let c = document.getElementById("message-history-container");return c.scrollHeight - c.offsetHeight - c.scrollTop"#,
    )
    .await
    {
        Ok(eval) => eval,
        Err(err) => {
            println!("Failed to eval scroll state getter: {}", err);
            return None;
        }
    };
    let Some(out) = eval.as_f64() else {
        println!("Failed to parse scroll state as f64: {}", eval);
        return None;
    };

    Some(out < AUTOSCROLL_THRESHOLD)
}

#[derive(Clone, PartialEq)]
struct UIChatMessage {
    message: ChatMessage,
    show_user: bool,
    show_time: bool,
}

fn combine_messages(messages: Vec<ChatMessage>) -> Vec<UIChatMessage> {
    if messages.is_empty() {
        return Vec::<UIChatMessage>::default();
    }

    let mut out = Vec::<UIChatMessage>::new();

    let mut prev: Option<&mut UIChatMessage> = None;
    let mut iter = messages.iter();

    while let Some(current) = iter.next() {
        let mut show_user = true;
        if let Some(prev) = prev {
            let cd = current.datetime().unwrap();
            let pt = prev.message.datetime().unwrap();

            // not perfect logic but should suffice
            let is_same_time =
                cd.minute() == pt.minute() && cd.hour() == pt.hour() && cd.day() == pt.day();

            if is_same_time && current.user == prev.message.user {
                prev.show_time = false;
                show_user = false;
            }
        }

        out.push(UIChatMessage {
            message: current.clone(),
            show_user: show_user,
            show_time: true,
        });

        prev = out.last_mut();
    }
    out
}

#[component]
pub fn MessageHistory(messages: Memo<Vec<ChatMessage>>) -> Element {
    let state = use_context::<AppState>();
    let username = state.username;

    let mut final_messages = use_signal(|| Vec::<UIChatMessage>::new());

    use_effect(move || {
        final_messages.set(combine_messages(messages()));

        spawn(async move {
            if should_autoscroll().await.unwrap_or(false) {
                scroll_to_anchor();
            }
        });

        println!("updating messages");
    });

    rsx! {
        div {
            id: "message-history-container",
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
            for message in final_messages.read().iter() {
                Message {
                    message: message.clone(),
                    is_me: message.message.user == username(),
                }
            }

            // used for autoscrolling when new messages are added
            div { id: "page-anchor", width: "100%", height: "1px" }
        }
    }
}
