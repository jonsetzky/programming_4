extern crate directories;
use std::{
    fs,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

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
    models::MessageModel,
    repository::{Channel, Message, Repository},
    sqlite_repository::{SqliteRepository, establish_connection, run_migrations},
    tcp_chat_client::{PacketBuilder, PacketType},
};
mod models;
mod poc_repo;
mod schema;
mod sqlite_repository;

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
        let repo = Arc::new(Mutex::new(SqliteRepository::new()));
        let client = TcpChatClient::new(packet_builder.clone(), repo.clone());
        return AppState {
            client,
            packet_builder,
            repo,
        };
    });

    // todo channels added from other clients arent yet added here
    let mut all_channels: Signal<Vec<Channel>> = use_signal(|| vec![]);
    let mut channel: Signal<Option<Channel>> = use_signal(|| None);
    let mut channel_input = use_signal(|| String::from(""));

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
        // messages.write().push(msg.clone());
        let repo = app.repo();
        let repo = repo.read();
        let repo = repo.lock().unwrap();
        repo.add_message(msg.clone());
        if let Some(chl) = channel.read().cloned()
            && msg.channel == chl.id
        {
            messages.write().push(msg);
        }
    };
    // let mut messages: Signal<Vec<String>> = use_signal(|| vec![]);

    let mut alert_message: Signal<Option<String>> = use_signal(|| None);
    let mut show_alert = use_signal(|| false);

    use_effect(move || {
        {
            all_channels.set(app.repo().read().lock().unwrap().get_channels());
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

    use_effect(move || {
        match channel.read().clone() {
            None => {
                messages.set(vec![]);
            }
            Some(chl) => {
                messages.set(app.repo().read().lock().unwrap().get_n_messages_before(
                    channel.read().as_ref().unwrap().id,
                    SystemTime::now().into(),
                    10usize.into(),
                ));
            }
        };
    });

    let is_connected = app.client().read().is_probably_connected();

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
                    span { "{c.name}" }
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
                button {
                    onclick: move |_| {
                        all_channels.set(app.repo().read().lock().unwrap().get_channels());
                    },
                    "refresh"
                }
                span { ": " }
                for channel in all_channels.read().iter() {
                    span { r#""{channel.name.clone()}","# }
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
                button {
                    disabled: channel_input.read().len() < 1,
                    onclick: move |_| {
                        let new_channel = Channel {
                            id: Uuid::new_v4(),
                            name: channel_input.read().cloned(),
                        };
                        {
                            app.repo().read().lock().unwrap().add_channels(vec![new_channel.clone()]);
                        }
                        channel_input.set(String::from(""));
                        all_channels.write().push(new_channel.clone());
                        channel.set(Some(new_channel.clone()));
                        let packet = app.packet_builder().read().respond_channels(vec![new_channel]);
                        app.client().read().send(packet);
                    },
                    "create"
                }
                button {
                    disabled: all_channels.read().iter().find(|c| c.name == *channel_input.read()).is_none(),
                    onclick: move |_| {
                        let new_channel = all_channels
                            .read()
                            .clone()
                            .into_iter()
                            .find(|c| c.name == *channel_input.read())
                            .unwrap();
                        {
                            app.repo().read().lock().unwrap().add_channels(vec![new_channel.clone()]);
                        }
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
                let packet = app
                    .packet_builder()
                    .read()
                    .clone()
                    .chat_message(channel.read().cloned().unwrap().id, message);
                add_message(Message::from_packet(&packet));
                app.client().read().send(packet);
                dioxus::core::needs_update();
            },
        }
        if channel.read().is_none() {
            p { "not in a channel, no history to show" }
        } else {
            MessageHistory { messages }
        }
        div {
            button { onclick: move |_| show_alert.set(true), "Show Alert" }
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

    use self::schema::messages::dsl::*;
    use diesel::prelude::*;
    let conn = &mut establish_connection();
    run_migrations(conn);

    let res = messages
        .select(MessageModel::as_select())
        .load(conn)
        .expect("err selecting messages");
    println!("Found {} messages", res.len());

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
