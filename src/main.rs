extern crate directories;
use directories::ProjectDirs;

use dioxus::prelude::*;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder};

#[component]
fn App() -> Element {
    let mut hello_world_count = use_signal(|| 0);

    rsx! {
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
}
