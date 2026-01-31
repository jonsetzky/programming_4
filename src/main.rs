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

#[derive(Clone, Debug, PartialEq, Routable)]
enum Route {
    #[route("/")]
    Login,
    // #[route("/home")]
    // Home,

    // #[route("/user/:id")]
    // User { id: u32 },
}

static RESET_CSS: Asset = asset!("/assets/reset.css");
static MAIN_CSS: Asset = asset!("/assets/main.css");

// todo use embedded font?
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: RESET_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: "https://fonts.googleapis.com/css?family=Inter" }
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
