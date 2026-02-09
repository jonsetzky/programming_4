use std::{collections::HashMap, io, time::Duration};

use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::sync::{mpsc::Receiver, oneshot};

// idea from https://stackoverflow.com/questions/59170011/why-the-result-of-regexnew-cannot-be-assigned-to-a-constant
lazy_static! {
    pub static ref JOIN_CHANNEL_STATUS_REGEX: Regex =
        Regex::new(r"^You joined the channel\s+(.+)$").unwrap();
}

use crate::{
    AppState,
    components::{
        Button, ChannelButton, MessageHistory, UserPanel,
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    packet::{ChatMessage, Packet},
    route::Route,
    tcp_chat_client::TcpChatClient,
};

pub fn on_recv_msg(
    mut messages: Signal<HashMap<String, Vec<ChatMessage>>>,
    active_channel: Signal<String>,
) -> impl FnMut(ChatMessage) -> () {
    move |message| {
        let mut messages = messages.write();
        let Some(existing_channel) = messages.get_mut(&active_channel()) else {
            messages.insert(active_channel(), vec![message]);
            return;
        };
        existing_channel.push(message);
    }
}

async fn client_connect_loop(
    mut connected: Signal<bool>,
    active_channel: Signal<String>,
    messages: Signal<HashMap<String, Vec<ChatMessage>>>,
) {
    let state = use_context::<AppState>();
    let mut connection_notification = state.connection_notification;
    let mut packet_sender = state.packet_sender;
    loop {
        connected.set(false);
        let client = match TcpChatClient::connect(Some(state.address.to_string().as_str())).await {
            Ok(client) => client,
            Err(_err) => {
                connection_notification.set(String::from("Error connecting to the server."));
                tokio::time::sleep(Duration::from_secs(5)).await;
                connected.set(false);
                continue;
            }
        };

        connected.set(true);
        connection_notification.set(String::from(""));

        let (sendTx, sendRx) = tokio::sync::mpsc::channel::<Packet>(100);
        packet_sender.set(Some(sendTx));

        let (tx, rx) = oneshot::channel::<()>();
        let _client = client.clone();
        let _read_handle = spawn(async move {
            read_loop(
                _client,
                active_channel,
                on_recv_msg(messages, active_channel),
            )
            .await;
            let _ = tx.send(()); // notify when read loop exits
        });

        let _client = client.clone();
        let _write_handle = spawn(async move {
            write_loop(_client, sendRx).await;
        });

        let _ = client.send(Packet::ListChannels { channels: None }).await;

        let _ = rx.await;
        _write_handle.cancel();
    }
}

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

async fn write_loop(client: TcpChatClient, mut outgoing_rx: Receiver<Packet>) {
    println!("starting write loop");
    loop {
        let Some(packet) = outgoing_rx.recv().await else {
            break;
        };
        println!(
            "sent packet {}",
            String::from_utf8(packet.into_bytes()).unwrap()
        );

        match client.send(packet).await {
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
}

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let state = use_context::<AppState>();
    let connected = use_signal(|| false);
    let packet_sender = state.packet_sender;

    let channels = state.channels;
    let active_channel = use_signal(|| String::from(""));
    // let packet_builder = state.packet_builder.clone();

    // let mut channel_messages: Signal<Vec<ChatMessage>> = use_signal(|| vec![]);
    let messages: Signal<HashMap<String, Vec<ChatMessage>>> =
        use_signal(|| HashMap::<String, Vec<ChatMessage>>::new());

    use_future(
        move || async move { client_connect_loop(connected, active_channel, messages).await },
    );

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
                    ChannelButton { active_channel, name_with_user_count: chl }
                }
                hr { align_self: "center" }
                Button { class: "add-neighborhood-button", label: "+ Add" }
                div { flex: "1" }
                hr { align_self: "center" }
                UserPanel { connected, username: state.username }
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
