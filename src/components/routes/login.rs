use dioxus::{html::h1, prelude::*};

#[component]
pub fn Login() -> Element {
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
            }
        }
    }
}
