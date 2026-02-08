#![feature(lock_value_accessors)]
extern crate directories;
use std::fs;

use dioxus::{html::u::background_color, prelude::*};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod components;
use components::*;

// mod o4_chat_client;
// use o4_chat_client::O4ChatClient;

mod arc_mutex_signal;

mod packet;
mod packet_builder;
mod tcp_chat_client;

mod route;

use tcp_chat_client::TcpChatClient;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::{packet::Packet, packet_builder::PacketBuilder, route::Route};
#[derive(Debug, Store, Clone)]
struct AppState {
    packet_builder: PacketBuilder,
    username: Signal<String>,
    address: Signal<String>,
    connection_notification: Signal<String>,
    channels: Signal<Vec<String>>,
    // send_channel: Signal<Option>,
    packet_sender: Signal<Option<Sender<Packet>>>,
}

impl AppState {
    #[inline]
    pub fn send(&self, packet: Packet) {
        let packet_sender = self.packet_sender;
        if packet_sender().is_none() {
            println!("Trying to send() while not connected!");
            return;
        }

        // todo handle error?
        let _ = packet_sender().unwrap().send(packet);
    }
}

static RESET_CSS: Asset = asset!("/assets/reset.css");
static MAIN_CSS: Asset = asset!("/assets/main.css");

// todo use embedded font?
#[component]
fn App() -> Element {
    use_context_provider(|| {
        let username = String::from("");
        // todo use actual user id
        let packet_builder = PacketBuilder::new(username.clone());
        return AppState {
            packet_builder,
            username: Signal::new(username),
            address: Signal::new(String::from("127.0.0.1:10000")),
            connection_notification: Signal::new(String::from("")),
            channels: Signal::new(vec![]),
            packet_sender: Signal::new(None),
        };
    });

    rsx! {
        document::Stylesheet { href: RESET_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: "https://fonts.googleapis.com/css?family=Inter" }
        Notification {}
        div {
            background_color: "#171717",
            color: "#EEEEEE",
            font_family: "Inter",
            width: "100vw",
            height: "100vh",
            margin: "0",
            Router::<Route> {}
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
