use dioxus::prelude::*;
use neighbor_chat::packet;
use tokio::sync::mpsc::Sender;

use crate::{
    AppState,
    packet::{ChatMessage, Packet},
    packet_builder::PacketBuilder,
};

fn send_message(packet_sender: Sender<Packet>, message: String) -> ChatMessage {
    let state = use_context::<AppState>();
    let packet_builder = state.packet_builder();
    let packet = packet_builder.chat_message(message);
    let msg = match &packet {
        Packet::Chat(msg) => msg.clone(),
        _ => panic!("unreachable code"),
    };
    spawn(async move {
        // todo handle errors?
        match packet_sender.send(packet).await {
            Ok(_) => {}
            Err(err) => {
                println!(
                    "Got error when sending packet down the mpsc channel! {}",
                    err,
                );
            }
        }
    });
    msg
}

#[component]
pub fn MessageBox(
    disabled: bool,
    add_message: Callback<ChatMessage>,
    active_channel: Signal<String>,
) -> Element {
    let state = use_context::<AppState>();
    let packet_sender = state.packet_sender;

    let mut message = use_signal(|| String::from(""));

    rsx! {
        div { display: "flex", justify_content: "center", width: "100%",
            div {
                display: "flex",
                flex_direction: "row",
                width: "100%",
                flex_grow: "1",
                justify_items: "center",
                align_items: "center",
                textarea {
                    disabled,
                    font_size: "14px",
                    rows: 1,
                    placeholder: format!("Message {}", active_channel()),
                    border_radius: "6px 0px 0px 6px",
                    padding_left: "1rem",
                    padding_right: "0rem",
                    vertical_align: "center",
                    value: message.read().cloned(),
                    oninput: move |event| {
                        message.set(event.value());
                    },
                    onkeypress: move |event| {
                        if event.key() == Key::Enter {
                            event.prevent_default();
                            match packet_sender() {
                                Some(packet_sender) => {
                                    let msg = send_message(packet_sender, message());
                                    add_message(msg);
                                    message.set(String::from(""));
                                }
                                None => {
                                    println!("cant send message because packet_sender is null");
                                }
                            }
                        }
                    },
                    "Message here"
                }
                button {
                    flex_grow: "0",
                    flex_shrink: "0",
                    width: "2.3rem",
                    height: "100%",
                    min_width: "16px",
                    padding_bottom: "10px",
                    border_radius: "0px 6px 6px 0px",

                    display: "flex",
                    align_items: "flex-end",
                    justify_content: "center",

                    disabled: disabled || message.read().len() == 0,
                    onclick: move |_| {
                        match packet_sender() {
                            Some(packet_sender) => {
                                let msg = send_message(packet_sender, message());
                                add_message(msg);
                                message.set(String::from(""));
                            }
                            None => {
                                println!("cant send message because packet_sender is null");
                            }
                        }
                    },

                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 122.56 122.88",
                        width: "18",
                        height: "18",
                        fill: "#ddd",

                        path {
                            fill_rule: "evenodd",
                            d: "M2.33,44.58,117.33.37a3.63,3.63,0,0,1,5,4.56l-44,115.61h0a3.63,3.63,0,0,1-6.67.28L53.93,84.14,89.12,33.77,38.85,68.86,2.06,51.24a3.63,3.63,0,0,1,.27-6.66Z",
                        }
                    }
                }
            }
        }
    }
}
