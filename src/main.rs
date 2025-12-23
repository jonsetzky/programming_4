extern crate directories;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use directories::ProjectDirs;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod components;
use components::*;

// mod o4_chat_client;
// use o4_chat_client::O4ChatClient;

mod arc_mutex_signal;

mod tcp_chat_client;
use tcp_chat_client::TcpChatClient;
use uuid::Uuid;

mod repository;
use crate::{
    repository::{Message, Repository},
    tcp_chat_client::{PacketBuilder, PacketType},
};
mod models;
mod schema;
// mod sqlite_repository;
mod poc_repo;
use poc_repo::POCRepo;

#[derive(Store)]
struct AppState {
    client: TcpChatClient,
    repo: Arc<Mutex<dyn Repository>>,
    packet_builder: PacketBuilder,
}

#[component]
fn App() -> Element {
    let app = use_store(|| {
        // todo use actual user id
        let packet_builder = PacketBuilder::new(Uuid::new_v4(), String::from("n/a"));
        let repo = Arc::new(Mutex::new(POCRepo::new()));
        let client = TcpChatClient::new(packet_builder.clone(), repo.clone());
        return AppState {
            client,
            packet_builder,
            repo,
        };
    });

    let mut username = use_signal(|| String::from("test user"));
    use_effect(move || {
        app.packet_builder().read().set_nickname(&username.read());
    });

    let mut messages = use_signal(move || {
        app.repo().read().lock().unwrap().get_n_messages_before(
            Uuid::new_v4(),
            SystemTime::now().into(),
            10usize.into(),
        )
    });
    let mut add_message = move |msg: Message| {
        messages.write().push(msg.clone());
        app.repo().read().lock().unwrap().add_message(msg);
    };
    // let mut messages: Signal<Vec<String>> = use_signal(|| vec![]);

    use_effect(move || {
        {
            for channel in app.repo().read().lock().unwrap().get_channels() {
                println!("generated channel {} ({})", channel.name, channel.id);
            }
        }

        spawn(async move {
            let mut sleep_duration = 200;
            loop {
                let client = app.client();
                let client = client.read();

                let mut rx = client.get_incoming_rx();
                spawn(async move {
                    while let Ok(ref msg) = rx.recv().await {
                        match &msg.payload {
                            PacketType::Message { .. } => add_message(Message::from_packet(msg)),
                            _ => println!("received unhandled PacketType"),
                        };
                    }
                });

                match client.connect().await {
                    Ok(_) => {
                        println!("connect finished");
                        sleep_duration = 200;
                    }
                    Err(err) => {
                        println!("connect fail {}", err);
                        sleep_duration = num::clamp(sleep_duration + sleep_duration / 2, 200, 5000);
                    }
                };

                tokio::time::sleep(Duration::from_millis(sleep_duration)).await;
            }
        });
    });

    let is_connected = app.client().read().is_probably_connected();

    rsx! {
        Timer {}
        div {
            span { "username" }
            input {
                disabled: !is_connected,
                r#type: "text",
                value: username.read().cloned(),
                oninput: move |event| { username.set(event.value()) },
            }
        }
        div {
            button {
                onclick: move |_| {
                    for channel in app.repo().read().lock().unwrap().get_channels() {
                        println!("Channel {}'s id is {}", channel.name, channel.id);
                    }
                },
                "print channels"
            }
        }
        p {}
        MessageBox {
            disabled: !is_connected,
            onsend: move |message: String| {
                if !is_connected {
                    println!("Can't send message, because not connected to server.");
                    return;
                }
                let packet = app.packet_builder().read().clone().chat_message(message);
                add_message(Message::from_packet(&packet));
                app.client().read().send(packet);
                dioxus::core::needs_update();
            },
        }
        MessageHistory { messages }
    }
}

fn main() {
    println!(
        "Data directory: {}",
        ProjectDirs::from("", "jonsetzky", "Neighbor Chat")
            .unwrap()
            .data_dir()
            .to_str()
            .unwrap()
    );

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
