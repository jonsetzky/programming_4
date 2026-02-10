use std::{collections::HashMap, io, time::Duration};

use dioxus::prelude::*;
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
        channel_button::ChannelButton, create_channel_button::CreateChannelButton,
        message_box::MessageBox, message_history::MessageHistory, popup::Popup,
        topic_editor::TopicEditor, user_panel::UserPanel,
    },
    packet::{ChatMessage, Packet},
    route::Route,
    tcp_chat_client::TcpChatClient,
};

pub fn get_channel_name(name_with_user_count: String) -> String {
    let split = name_with_user_count.split(" ").collect::<Vec<&str>>();
    split[..split.len() - 1].join(" ")
}

pub fn add_message_to_messages(
    mut messages: Signal<HashMap<String, Vec<ChatMessage>>>,
    active_channel: Signal<String>,
) -> impl FnMut(ChatMessage) {
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
    topic: Signal<String>,
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

        let (send_tx, send_rx) = tokio::sync::mpsc::channel::<Packet>(100);
        packet_sender.set(Some(send_tx));

        let (tx, rx) = oneshot::channel::<()>();
        let _client = client.clone();
        let _read_handle = spawn(async move {
            read_loop(
                _client,
                active_channel,
                add_message_to_messages(messages, active_channel),
                topic,
            )
            .await;
            let _ = tx.send(()); // notify when read loop exits
        });

        let _client = client.clone();
        let _write_handle = spawn(async move {
            write_loop(_client, send_rx).await;
        });

        let _ = client.send(Packet::ListChannels { channels: None }).await;

        let _ = rx.await;
        _write_handle.cancel();
    }
}

async fn read_loop(
    mut client: TcpChatClient,
    mut active_channel: Signal<String>,
    mut add_message: impl FnMut(ChatMessage),
    mut topic: Signal<String>,
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
                consume_context::<AppState>().channels.set(
                    channels
                        .iter()
                        .map(|chl| get_channel_name(chl.to_string()))
                        .collect(),
                );
            }
            Packet::ChangeTopic { topic: new_topic } => {
                println!("NEW TOPIC: {}", new_topic);
                topic.set(new_topic);
            }
            Packet::Chat(message) => {
                println!("MESSAGE: [{}]: {}", message.user, message.message);
                add_message(message)
            }
            Packet::Error {
                error,
                clientshutdown: _,
            } => {
                println!("got error packet!: {}", error);
            }
            Packet::Status { status } => {
                if let Some(caps) = JOIN_CHANNEL_STATUS_REGEX.captures(status.as_str()) {
                    let channel_name = &caps[1];
                    println!("STATUS: updated current channel to {}", channel_name);
                    active_channel.set(channel_name.into());
                } else {
                    println!("STATUS: {}", status);
                }
            }
            Packet::JoinChannel { channel: _ } => {
                println!("received JoinChannel packet from server. weird..")
            }
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
            String::from_utf8(packet.to_bytes()).unwrap()
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

    let show_initial_popup = use_signal(|| true);

    let channels = state.channels;
    let active_channel = use_signal(|| String::from(""));
    let topic = use_signal(|| String::from(""));

    let messages: Signal<HashMap<String, Vec<ChatMessage>>> =
        use_signal(HashMap::<String, Vec<ChatMessage>>::new);

    let channel_messages = use_memo(move || {
        if let Some(msgs) = messages.get(&active_channel()) {
            msgs.cloned()
        } else {
            vec![]
        }
    });

    use_future(move || async move {
        client_connect_loop(connected, active_channel, messages, topic).await
    });

    rsx! {
        if show_initial_popup() {
            Popup { show: show_initial_popup,
                p { font_size: "32px", "Important notice" }
                div { height: "2rem" }
                p { font_size: "14px",
                    "Messages can only be received and viewed during the session. Logging out will erase all messages locally."
                }
            }
        }
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
                    ChannelButton { active_channel, name: chl }
                }
                hr { align_self: "center" }
                CreateChannelButton {
                }
                div { flex: "1" }
                hr { align_self: "center" }
                UserPanel { connected, username: state.username }
            }
            div {
                display: "flex",
                justify_content: "center",
                width: "100%",
                height: "100%",
                div {
                    // topiceditor requires this as it has an absolute child
                    // could possibly be moved into the parent div in TopicEditor??
                    position: "relative",
                    display: "flex",
                    flex_direction: "column",
                    flex: "1",
                    min_height: "0",
                    align_items: "center",
                    TopicEditor { topic }
                    div {
                        display: "flex",
                        flex_direction: "column",
                        width: "36rem",
                        flex: "1",
                        flex_shrink: "0",
                        min_height: "0",
                        justify_content: "center",
                        align_items: "center",
                        MessageHistory { messages: channel_messages }
                        div { flex: "1" }
                        MessageBox {
                            disabled: false,
                            add_message: add_message_to_messages(messages, active_channel),
                            active_channel,
                        }
                        div { height: "0.4rem" }
                    }
                }
            }
        }
    }
}
