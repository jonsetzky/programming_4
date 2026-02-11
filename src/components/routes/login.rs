use dioxus::prelude::*;

use crate::{
    AppState,
    components::{button::Button, input_field::InputField},
    route::Route,
};

#[component]
pub fn Login() -> Element {
    let mut state = use_context::<AppState>();
    let nav = navigator();
    let name: Signal<String> = use_signal(|| state.username.to_string());
    let address: Signal<String> = use_signal(|| state.address.to_string());
    use_effect(move || state.connection_notification.set("".into()));

    rsx! {
        div {
            width: "32rem",
            height: "100vh",
            text_align: "center",
            vertical_align: "middle",
            margin_left: "auto",
            margin_right: "auto",
            background_color: "#262626",
            align_content: "center",
            div { height: "16rem", width: "100%", margin: "auto",
                h1 { font_size: "48px", font_weight: "900", "Welcome" }
                form {
                    display: "flex",
                    flex_direction: "column",
                    padding_top: "3rem",
                    gap: "1rem",
                    align_items: "center",
                    justify_content: "center",
                    InputField {
                        label: "Full Name",
                        placeholder: "Firstname Lastname",
                        legal_regex: r"^[a-öA-Ö][a-öA-Ö\s]*$",
                        onillegal: |_| {
                            let mut state = consume_context::<AppState>();
                            state
                                .connection_notification
                                .set(
                                    "Name must start with a letter and contain only letters or spaces."
                                        .to_string(),
                                );
                        },
                        value: name,
                    }
                    InputField {
                        label: "Server Address",
                        placeholder: "127.0.0.1:10000",
                        value: address,
                    }

                    Button {
                        label: "Continue",
                        onclick: move |_| {
                            let name = name().trim().to_string();

                            if name.len() < 3 {
                                let mut state = consume_context::<AppState>();
                                state
                                    .connection_notification
                                    .set("Name must be longer than 3 characters.".to_string());
                                return;
                            }
                            if name.len() >= 3 {
                                state.packet_builder.set_nickname(&name);
                                state.username.set(name);
                                state.address.set(address());
                                state.connection_notification.set("".to_string());
                                nav.replace(Route::Home);
                            }
                        },
                    }
                }
            }
        }
    }
}
