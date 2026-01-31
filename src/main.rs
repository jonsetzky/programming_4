#![feature(lock_value_accessors)]
extern crate directories;
use std::fs;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod components;
use components::*;

// mod o4_chat_client;
// use o4_chat_client::O4ChatClient;

mod arc_mutex_signal;

mod packet;
mod packet_builder;
mod tcp_chat_client;

use tcp_chat_client::TcpChatClient;
use uuid::Uuid;

use crate::{
    packet::{ChatMessage, Packet},
    packet_builder::PacketBuilder,
};

#[derive(Store)]
struct AppState {
    packet_builder: PacketBuilder,
}

#[component]
fn App() -> Element {
    let app = use_store(|| {
        // todo use actual user id
        let packet_builder = PacketBuilder::new(format!(
            "test_user{:08}",
            rand::random::<u32>() % 100_000_000
        ));
        return AppState { packet_builder };
    });

    // todo channels added from other clients arent yet added here
    let mut channel_input = use_signal(|| String::from(""));
    let mut all_channels: Signal<Vec<String>> = use_signal(|| vec![]);
    let mut channel: Signal<Option<String>> = use_signal(|| None);

    let mut username = use_signal(|| String::from("test user"));
    use_effect(move || {
        app.packet_builder().read().set_nickname(&username.read());
    });

    let mut messages = use_signal(move || vec![]);
    let mut add_message = move |msg: ChatMessage| {
        messages.write().push(msg);
        // todo send to server
    };

    let mut alert_message: Signal<Option<String>> = use_signal(|| None);
    let mut show_alert = use_signal(|| false);

    // use_effect(move || {
    //     {
    //         all_channels.set(app.repo().read().lock().unwrap().get_channels());
    //     }

    //     spawn(async move {
    //         let mut sleep_duration = 200;
    //         loop {
    //             let client = app.client();
    //             let client = client.read();

    //             let mut rx = client.get_incoming_rx();
    //             spawn(async move {
    //                 while let Ok(ref msg) = rx.recv().await {
    //                     match &msg.payload {
    //                         PacketType::Message { .. } => add_message(Message::from_packet(msg)),
    //                         _ => println!("received unhandled PacketType"),
    //                     };
    //                 }
    //             });

    //             match client.connect().await {
    //                 Ok(_) => {
    //                     println!("connect finished");
    //                     sleep_duration = 200;
    //                 }
    //                 Err(err) => {
    //                     println!("connect fail {}", err);
    //                     sleep_duration = num::clamp(sleep_duration + sleep_duration / 2, 200, 5000);
    //                 }
    //             };

    //             tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
    //         }
    //     });
    // });

    // use_effect(move || {
    //     match channel.read().clone() {
    //         None => {
    //             messages.set(vec![]);
    //         }
    //         Some(chl) => {
    //             messages.set(app.repo().read().lock().unwrap().get_n_messages_before(
    //                 channel.read().as_ref().unwrap().id,
    //                 SystemTime::now().into(),
    //                 10usize.into(),
    //             ));
    //         }
    //     };
    // });

    // let is_connected = app.client().read().is_probably_connected();
    let is_connected = true;

    rsx! {
        Timer {}
        div {
            h3 { "username" }
            span { "username" }
            input {
                disabled: !is_connected,
                r#type: "text",
                value: username.read().cloned(),
                oninput: move |event| { username.set(event.value()) },
            }
        }
        div {
            h3 { "channels" }
            p {
                span { "current channel: " }
                if let Some(c) = channel.read().clone() {
                    span { "{c}" }
                } else {
                    span { "none " }
                }
                button {
                    disabled: channel.read().is_none(),
                    onclick: move |_| channel.set(None),
                    "leave"
                }
            }
            p {
                span { "channels available" }
                button { onclick: move |_| {}, // all_channels.set(app.repo().read().lock().unwrap().get_channels());, "refresh" }
                    span { ": " }
                    for chl in all_channels.read().clone().into_iter() {
                        span { r#"""# }
                        span {
                            cursor: "pointer",
                            onclick: move |_| {
                                channel.set(Some(chl.clone()));
                            },
                            r#"{chl}"#
                        }
                        span { r#"","# }
                    }
                }
                div {
                    span { "create/change a channel" }
                    input {
                        // disabled: channel_input.read().len() < 1,
                        r#type: "text",
                        value: channel_input.read().cloned(),
                        oninput: move |event| { channel_input.set(event.value()) },
                    }
                    // button {
                    //     disabled: channel_input.read().len() < 1,
                    //     onclick: move |_| {
                    //         let new_channel = Channel {
                    //             id: Uuid::new_v4(),
                    //             name: channel_input.read().cloned(),
                    //         };
                    //         {
                    //             app.repo().read().lock().unwrap().add_channels(vec![new_channel.clone()]);
                    //         }
                    //         channel_input.set(String::from(""));
                    //         all_channels.write().push(new_channel.clone());
                    //         channel.set(Some(new_channel.clone()));
                    //         let packet = app.packet_builder().read().respond_channels(vec![new_channel]);
                    //         app.client().read().send(packet);
                    //     },
                    //     "create"
                    // }
                    button {
                        disabled: all_channels.read().iter().find(|c| **c == *channel_input.read()).is_none(),
                        onclick: move |_| {
                            let new_channel = all_channels
                                .read()
                                .clone()
                                .into_iter()
                                .find(|c| *c == *channel_input.read())
                                .unwrap();
                            // todo change channel message to server
                            channel.set(Some(new_channel.clone()));
                            channel_input.set(String::from(""));
                        },
                        "change to"
                    }
                }
            }
            h3 { "chat" }
            MessageBox {
                disabled: !is_connected || channel.read().is_none(),
                onsend: move |message: String| {
                    if !is_connected {
                        println!("Can't send message, because not connected to server.");
                        return;
                    }
                    if channel.read().is_none() {
                        println!("Can't send message because no channel is selected");
                        return;
                    }
                    let packet: Packet = app
                        .packet_builder()
                        .read()
                        .clone()
                        .chat_message(message);

                    match packet {
                        Packet::Chat(msg) => add_message(msg),
                        _ => panic!("Expected a chat packet"),
                    };
                    dioxus::core::needs_update();
                },
            }
            if channel.read().is_none() {
                p { "not in a channel, no history to show" }
            } else {
                MessageHistory { messages }
            }
        }
    }
}

fn main() {
    if let Err(err) = fs::create_dir_all(neighbor_chat::data_dir()) {
        panic!("error creating data directory {}", err);
    } else {
        println!(
            "Data directory: {}",
            neighbor_chat::data_dir().to_str().unwrap()
        );
    }

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop! {
           Config::new().with_window(
               WindowBuilder::new()
                    .with_maximizable(false)
                    // .with_decorations(false)
                    .with_always_on_top(false)
                    .with_inner_size(LogicalSize {width: 1280, height: 720})
                    .with_title("Neighbor Chat")
           )
        })
        .launch(App);

    println!("Launched app!");
}
