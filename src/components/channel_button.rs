use crate::packet::Packet;
use crate::{AppState, components::Button};
use dioxus::prelude::*;

#[component]
pub fn ChannelButton(name: String, active_channel: Signal<String>) -> Element {
    let state = use_context::<AppState>();
    let packet_sender = state.packet_sender;

    let is_active_channel = name == active_channel();

    rsx! {
        Button {
            disabled: is_active_channel,
            class: if is_active_channel { "neighborhood-button-current" } else { "neighborhood-button" },
            label: name.clone(),
            onclick: move |_evt| {
                let chl_name = name.clone();
                if chl_name == active_channel() {
                    return;
                }
                spawn(async move {
                    match packet_sender
                        .unwrap()
                        .send(Packet::JoinChannel {
                            channel: chl_name,
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            println!("Failed to send packet down the mpsc channel: {}", err);
                        }
                    };
                });
            },
        }
    }
}
