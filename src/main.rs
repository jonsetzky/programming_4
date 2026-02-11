#![feature(lock_value_accessors)]
extern crate directories;
use std::fs;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder, use_window};

mod components;
mod packet;
mod packet_builder;
mod tcp_chat_client;

mod route;

use tokio::sync::mpsc::Sender;

use crate::{
    components::notification::Notification, packet::Packet, packet_builder::PacketBuilder,
    route::Route,
};
#[derive(Debug, Store, Clone)]
struct AppState {
    packet_builder: PacketBuilder,
    username: Signal<String>,
    address: Signal<String>,
    connection_notification: Signal<String>,
    channels: Signal<Vec<String>>,
    packet_sender: Signal<Option<Sender<Packet>>>,
}

impl AppState {
    pub fn packet_builder(&self) -> PacketBuilder {
        self.packet_builder.clone()
    }

    pub fn new() -> AppState {
        let username = String::from("");
        let packet_builder = PacketBuilder::new(username.clone());
        AppState {
            packet_builder,
            username: Signal::new(username),
            address: Signal::new(String::from("127.0.0.1:10000")),
            connection_notification: Signal::new(String::from("")),
            channels: Signal::new(vec![]),
            packet_sender: Signal::new(None),
        }
    }
}

static RESET_CSS: Asset = asset!("/assets/reset.css");
static MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn ExitButton() -> Element {
    let window = use_window();

    rsx! {
        div {
            class: "decoration-exit-button",
            cursor: "pointer",
            width: "24px",
            height: "24px",
            onclick: move |evt| {
                evt.stop_propagation();
                evt.prevent_default();

                window.close();
            },
            svg {
                class: "w-6 h-6 text-gray-800 dark:text-white",
                fill: "none",
                height: "24",
                view_box: "0 0 24 24",
                width: "24",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M6 18 17.94 6M18 18 6.06 6",
                    stroke: "#aaa",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1",
                }
            }
        }
    }
}

#[component]
fn MinimizeButton() -> Element {
    let window = use_window();

    rsx! {

        div {
            class: "decoration-button",
            cursor: "pointer",
            width: "24px",
            height: "24px",
            onclick: move |evt| {
                evt.stop_propagation();
                evt.prevent_default();

                window.set_minimized(true);
            },
            svg {
                class: "w-6 h-6 text-gray-800 dark:text-white",
                fill: "none",
                height: "24",
                view_box: "0 0 24 24",
                width: "24",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M6 18 18 18",
                    stroke: "#aaa",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "1",
                }
            }
        }
    }
}

#[component]
fn App() -> Element {
    let window = use_window();

    use_context_provider(AppState::new);

    rsx! {
        document::Stylesheet { href: RESET_CSS }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: "https://fonts.googleapis.com/css?family=Inter" }
        div {
            background_color: "#171717",
            color: "#EEEEEE",
            font_family: "Inter",
            display: "flex",
            flex_direction: "column",
            height: "100vh",
            width: "100vw",
            Notification {}
            div {
                position: "absolute",
                top: "0",
                right: "0",
                padding: "3px 3px 0px 0px",
                z_index: 5,
                display: "flex",
                flex_direction: "row",
                MinimizeButton {}
                ExitButton {}
            }
            div {
                width: "100%",
                height: "1.8rem",
                background_color: "#202020",
                color: "white",
                flex_shrink: "0",
                z_index: 4,
                onmousedown: move |_| window.clone().drag(),
            }
            div { flex: "1", margin: "0", Router::<Route> {} }
        }
    }
}

fn main() {
    if let Err(err) = fs::create_dir_all(neighbor_chat::data_dir()) {
        panic!("error creating data directory {}", err);
    }
    let data_dir = neighbor_chat::data_dir();

    println!("Data directory: {}", data_dir.to_str().unwrap());

    dioxus::LaunchBuilder::new()
        .with_cfg(desktop! {
           Config::new().with_window(
               WindowBuilder::new()
                    .with_maximizable(false)
                    .with_decorations(false)
                    .with_always_on_top(false)
                    .with_inner_size(LogicalSize {width: 900, height: 720})
                    .with_title("Neighbor Chat")
                    .with_min_inner_size(LogicalSize {width: 920, height: 520})
           ).with_close_behaviour(dioxus_desktop::WindowCloseBehaviour::WindowCloses)
           .with_data_directory(data_dir)
        })
        .launch(App);

    println!("Launched app!");
}
