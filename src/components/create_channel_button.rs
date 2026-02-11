use crate::{AppState, components::button::Button};
use dioxus::prelude::*;

#[component]
pub fn CreateChannelButton() -> Element {
    let mut show_input = use_signal(|| false);
    let mut value = use_signal(|| String::from(""));

    if show_input() {
        rsx! {
            div {
                class: "add-neighborhood-button",
                width: "100%",
                display: "flex",
                align_items: "center",
                flex_direction: "row",
                justify_content: "space-between",
                padding: "0px 0px 0px 0px", // trbl
                border_radius: "0px",
                gap: "2px",
                onblur: move |_| {
                    show_input.set(false);
                },
                input {
                    padding: "0px 4px 0px 4px", // trbl
                    border_radius: "0px",
                    r#type: "text",
                    autofocus: true,
                    oninput: move |event| {
                        value.set(event.value());
                    },
                    value,
                    placeholder: "A channel",
                }
                button {
                    justify_self: "end",
                    flex_grow: "0",
                    flex_shrink: "0",
                    width: "2.3rem",
                    height: "100%",
                    min_width: "16px",
                    padding_bottom: "10px",
                    border_radius: "0px",

                    display: "flex",
                    align_items: "flex-end",
                    justify_content: "center",
                    onclick: move |evt| {
                        let state = consume_context::<AppState>();

                        let new_channel = value();
                        evt.prevent_default();
                        let Some(packet_sender) = (state.packet_sender)() else {
                            println!("cant send message because packet_sender is null");
                            return;
                        };
                        let packet = state
                            .packet_builder
                            .join_channel(new_channel.clone());

                        spawn(async move {
                            match packet_sender.send(packet).await {
                                Ok(_) => {
                                    let mut channels = consume_context::<AppState>().channels;
                                    println!("created new channel {}", new_channel.clone());

                                    if !channels().contains(&new_channel) {
                                        channels.write().push(new_channel);
                                    }
                                    value.set(String::from(""));
                                    show_input.set(false);
                                }
                                Err(err) => {
                                    println!("Failed to send packet down the mpsc channel: {}", err);
                                }
                            }
                        });
                    },
                    "+"
                }
            }
        }
    } else {
        rsx! {
            Button {
                class: "add-neighborhood-button",
                label: "+ Add",
                onclick: move |_| {
                    show_input.set(true);
                },
            }
        }
    }
}
