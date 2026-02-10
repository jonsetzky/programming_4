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

#[component]
pub fn MessageHistory(messages: Memo<Vec<ChatMessage>>) -> Element {
    let state = use_context::<AppState>();
    let username = state.username;

    use_effect(move || {
        // run this effect every time messages update
        messages();

        spawn(async move {
            if should_autoscroll().await.unwrap_or(false) {
                scroll_to_anchor();
            }
        });
    });

    // todo join continuous messages from same sender during same minute into one block

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
            for message in messages().iter() {
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
