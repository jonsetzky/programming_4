use dioxus::{html::h1, prelude::*};

use crate::{
    components::{Button, InputField},
    route::Route,
};

#[component]
pub fn Login() -> Element {
    let nav = navigator();
    let name: Signal<String> = use_signal(|| String::from(""));

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
                        value: name,
                    }
                    Button {
                        label: "Continue",
                        onclick: move |_| {
                            nav.replace(Route::Home);
                        },
                    }
                }
            }
        }
    }
}
