use crate::packet::Packet;
use crate::{
    AppState,
    components::{
        Button,
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
};
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};
use tokio::sync::mpsc::Sender;

pub fn get_channel_name(name_with_user_count: String) -> String {
    let split = name_with_user_count.split(" ").collect::<Vec<&str>>();
    split[..split.len() - 1].join(" ")
}

#[component]
pub fn ChannelButton(name_with_user_count: String, active_channel: Signal<String>) -> Element {
    let state = use_context::<AppState>();
    let packet_sender = state.packet_sender;

    let channel_name = get_channel_name(name_with_user_count);
    let is_active_channel = channel_name == active_channel();

    rsx! {
        Button {
            disabled: is_active_channel,
            class: if is_active_channel { "neighborhood-button-current" } else { "neighborhood-button" },
            label: channel_name.clone(),
            onclick: move |_evt| {
                let chl_name = channel_name.clone();
                if chl_name == active_channel() {
                    return;
                }
                spawn(async move {
                    // todo handle errors?
                    match packet_sender
                        .unwrap()
                        .send(Packet::JoinChannel {
                            channel: chl_name,
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(err) => {
                            println!(
                                "Got error when sending packet down the mpsc channel! {}",
                                err,
                            );
                        }
                    }
                });
            },
        }
    }
}
