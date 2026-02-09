use std::{collections::HashMap, io, time::Duration};

use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::sync::oneshot;

// idea from https://stackoverflow.com/questions/59170011/why-the-result-of-regexnew-cannot-be-assigned-to-a-constant
lazy_static! {
    pub static ref JOIN_CHANNEL_STATUS_REGEX: Regex =
        Regex::new(r"^You joined the channel\s+(.+)$").unwrap();
}

use crate::{
    AppState,
    components::{
        Button, MessageHistory,
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    packet::{ChatMessage, Packet},
    route::Route,
    tcp_chat_client::TcpChatClient,
};

async fn read_loop(
    mut client: TcpChatClient,
    mut active_channel: Signal<String>,
    mut add_message: impl FnMut(ChatMessage) -> (),
) {
    loop {
        let packet = match client.recv().await {
            Err(err) => {
                if err.kind() == io::ErrorKind::ConnectionAborted {
                    break;
                } else {
                    println!("unknown error while attempting to recv()");
                    break;
                }
            }
            Ok(packet) => packet,
        };
        // println!(
        //     "got packet {}",
        //     String::from_utf8(packet.into_bytes()).unwrap()
        // );

        match packet {
            Packet::ListChannels { channels } => {
                let Some(channels) = channels else {
                    continue;
                };
                consume_context::<AppState>().channels.set(channels);
            }
            Packet::ChangeTopic { topic } => {
                //todo handle
                println!("NEW TOPIC: {}", topic);
            }
            Packet::Chat(message) => {
                println!("MESSAGE: [{}]: {}", message.user, message.message);
                add_message(message)
            }
            Packet::Error {
                error,
                clientshutdown,
            } => {
                println!("got error packet!: {}", error);
            }
            Packet::Status { status } => {
                if let Some(caps) = JOIN_CHANNEL_STATUS_REGEX.captures(status.as_str()) {
                    let channel_name = &caps[1];
                    active_channel.set(channel_name.into());
                } else {
                    println!("STATUS: {}", status);
                }
            }
            Packet::JoinChannel { channel } => {
                // todo handle
                println!("received JoinChannel packet from server. weird..")
            }
            _ => println!("unhandled packet type"),
        }
    }
}

pub fn get_channel_name(name_with_user_count: String) -> String {
    let split = name_with_user_count.split(" ").collect::<Vec<&str>>();
    split[..split.len() - 1].join(" ")
}

const EMPTY_VEC: Vec<ChatMessage> = vec![];
#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let state = use_context::<AppState>();
    let mut connection_notification = state.connection_notification;
    let mut connected = use_signal(|| false);
    let mut packet_sender = state.packet_sender;

    let channels = state.channels;
    let active_channel = use_signal(|| String::from(""));
    // let packet_builder = state.packet_builder.clone();

    // let mut channel_messages: Signal<Vec<ChatMessage>> = use_signal(|| vec![]);
    let mut messages: Signal<HashMap<String, Vec<ChatMessage>>> =
        use_signal(|| HashMap::<String, Vec<ChatMessage>>::new());

    use_future(move || async move {
        loop {
            connected.set(false);
            let client = match TcpChatClient::connect(Some(state.address.to_string().as_str()))
                .await
            {
                Ok(client) => client,
                Err(err) => {
                    connection_notification.set(String::from("Error connecting to the server."));
                    // println!("error connecting. attempting again in 5 seconds");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    connected.set(false);
                    continue;
                }
            };

            connected.set(true);
            connection_notification.set(String::from(""));

            let (sendTx, mut sendRx) = tokio::sync::mpsc::channel::<Packet>(100);
            packet_sender.set(Some(sendTx));

            let (tx, rx) = oneshot::channel::<()>();
            let _client = client.clone();
            let _read_handle = spawn(async move {
                read_loop(_client, active_channel, move |message| {
                    let mut messages = messages.write();
                    let Some(existing_channel) = messages.get_mut(&active_channel()) else {
                        messages.insert(active_channel(), vec![message]);
                        return;
                    };
                    existing_channel.push(message);
                })
                .await;
                let _ = tx.send(()); // notify when read loop exits
            });

            let _client = client.clone();
            let _write_handle = spawn(async move {
                println!("starting write loop");
                loop {
                    let Some(packet) = sendRx.recv().await else {
                        break;
                    };
                    println!(
                        "sent packet {}",
                        String::from_utf8(packet.into_bytes()).unwrap()
                    );

                    match _client.send(packet).await {
                        Err(err) => {
                            if err.kind() == io::ErrorKind::ConnectionAborted {
                                break;
                            } else {
                                println!("unknown error while attempting to send()");
                                break;
                            }
                        }
                        Ok(packet) => packet,
                    };
                }
                println!("write loop exited")
            });

            let _ = client.send(Packet::ListChannels { channels: None }).await;

            let _ = rx.await;
            _write_handle.cancel();
        }
    });

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100vw",
            height: "100vh",
            justify_items: "start",
            div {
                display: "flex",
                flex_direction: "column",
                height: "100vh",
                width: "20rem",
                background_color: "#262626",
                flex_shrink: "0",
                // align_items: "center",
                gap: "4px",
                h2 {
                    onclick: move |_| {
                        nav.replace(Route::Login);
                    },
                    padding: "1rem",
                    padding_top: "1.2rem",
                    "Your Neighborhoods"
                }
                hr { align_self: "center" }
                for chl in channels() {
                    Button {
                        class: if get_channel_name(chl.clone()) == active_channel() { "neighborhood-button-current" } else { "neighborhood-button" },
                        label: get_channel_name(chl.clone()),
                        onclick: move |_evt| {
                            let chl = chl.clone();
                            spawn(async move {
                                let chl_name = get_channel_name(chl);

                                // todo handle errors?
                                match packet_sender
                                    .unwrap()
                                    .send(Packet::JoinChannel {
                                        channel: chl_name,
                                    })
                                    .await
                                {
                                    Ok(_) => {}
                                    Err(err) => {
                                        println!(
                                            "Got error when sending packet down the mpsc channel! {}",
                                            err,
                                        );
                                    }
                                }
                            });
                        },
                    }
                }
                hr { align_self: "center" }
                Button { class: "add-neighborhood-button", label: "+ Add" }
                div { flex: "1" }
                hr { align_self: "center" }
                div {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    justify_items: "start",
                    Button { class: "user-button", label: state.username }
                    Tooltip { height: "100%",
                        TooltipTrigger { height: "100%",
                            div {
                                display: "flex",
                                align_items: "center",
                                height: "100%",
                                div {
                                    width: "6px",
                                    height: "6px",
                                    background_color: if connected() { "green" } else { "red" },
                                    border_radius: "50%",
                                    margin_left: "4px", // Add some space between the button and the circle
                                    margin_right: "1.5rem",
                                    align_self: "center",
                                }
                            }
                        }
                        TooltipContent {
                            // The side of the TooltipTrigger where the content will be displayed. Can be one of Top, Right, Bottom, or Left.
                            side: ContentSide::Right,
                            // The alignment of the TooltipContent relative to the TooltipTrigger. Can be one of Start, Center, or End.
                            align: ContentAlign::Center,
                            style: "background-color: #000;color: #fff",
                            // The content of the tooltip, which can include text, images, or any other elements.
                            p { style: "margin: 0;",
                                if connected() {
                                    "Online"
                                } else {
                                    "Offline"
                                }
                            }
                        }
                    }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                width: "100%",
                height: "100%",
                flex_grow: "0",
                justify_items: "center",
                align_items: "center",
                MessageHistory { messages: messages.get(&active_channel()).map(|m| m.clone()).unwrap_or_default() }
            }
        }
    }
}
