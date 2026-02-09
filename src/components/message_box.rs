use dioxus::prelude::*;
use neighbor_chat::packet;
use tokio::sync::mpsc::Sender;

use crate::{
    AppState,
    packet::{ChatMessage, Packet},
    packet_builder::PacketBuilder,
};

fn send_message(
    packet_builder: PacketBuilder,
    packet_sender: Sender<Packet>,
    message: String,
) -> ChatMessage {
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
pub fn MessageBox(disabled: bool, add_message: Callback<ChatMessage>) -> Element {
    let state = use_context::<AppState>();
    let packet_sender = state.packet_sender;
    let packet_builder = state.packet_builder;

    let mut message = use_signal(|| String::from(""));

    rsx! {
        input {
            disabled,
            r#type: "text",
            value: message.read().cloned(),
            oninput: move |event| { message.set(event.value()) },
            "Message here"
        }
        button {
            disabled: disabled || message.read().len() == 0,
            onclick: move |_| {
                match packet_sender() {
                    Some(packet_sender) => {
                        let msg = send_message(packet_builder.clone(), packet_sender, message());
                        add_message(msg);
                    }
                    None => {
                        println!("cant send message because packet_sender is null");
                    }
                }
            },
            "Send"
        }
    }
}
