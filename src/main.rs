extern crate directories;
use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use std::time::SystemTime;

use dioxus::{prelude::*, stores::use_store_sync};
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

mod o4_chat_client;
use o4_chat_client::O4ChatClient;

#[component]
fn Timer() -> Element {
    let mut time = use_signal(SystemTime::now);

    use_effect(move || {
        spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                *time.write() = SystemTime::now();
            }
        });
    });

    let datetime: DateTime<Utc> = (*time.read()).into();
    let time_str = datetime.to_rfc3339_opts(chrono::SecondsFormat::Millis, false);

    rsx!(
        p { "{time_str}" }
    )
}

#[derive(Store)]
struct AppState {
    client: O4ChatClient,
}

#[component]
fn App() -> Element {
    let app = use_store_sync(|| AppState {
        client: O4ChatClient::new(None).expect("Couldn't create chat client"),
    });
    let mut hello_world_count = use_signal(|| 0);
    let mut is_connected = use_signal(|| false);

    let connect = move || {
        spawn(async move {
            if app.client().read().is_connected() {
                return;
            }
            app.client()
                .write()
                .connect()
                .await
                .expect("Failed to connect");
            if app.client().read().is_connected() {
                is_connected.set(true);
            }
        });
    };

    let mut disconnect = move || {
        let mut client = app.client();
        if !app.client().read().is_connected() {
            return;
        }
        match client.write().disconnect() {
            Ok(_) => is_connected.set(false),
            Err(err) => println!("Error disconnecting from server {}", err),
        }
    };

    use_effect(connect);

    rsx! {
        Timer {}
        if *is_connected.read() {
            button {
                onclick: move |_| {
                    disconnect();
                },
                "Disconnect"
            }
        } else {
            p { "NOT CONNECTED TO SERVER" }
            button {
                onclick: move |_| {
                    connect();
                },
                "Connect"
            }
        }
        button { onclick: move |_| hello_world_count += 1, "Ask for more Hello, World!" }
        for _ in 0..*hello_world_count.read() {
            p { "Hello, World!" }
        }
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
                    .with_decorations(false)
                    .with_always_on_top(false)
                    .with_inner_size(LogicalSize {width: 1280, height: 720})
                    .with_title("Neighbor Chat")
           )
        })
        .launch(App);

    println!("Launched app!");
}
