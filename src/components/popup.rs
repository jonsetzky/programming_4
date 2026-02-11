use dioxus::prelude::*;

#[component]
pub fn Popup(children: Element, show: Signal<bool>) -> Element {
    rsx! {
        div {
            z_index: "2",
            position: "fixed",
            height: "100vh",
            width: "100vw",
            background_color: "rgba(0, 0, 0, 0.32)",
            display: "flex",
            justify_content: "center",
            align_items: "center",
            div {
                background_color: "#262626",
                width: "24rem",
                height: "16rem",
                display: "flex",
                flex_direction: "column",
                padding: "2rem",
                {children}
                div { flex: "1" }
                div { display: "flex", flex_direction: "row", width: "100%",
                    div { flex: "1" }
                    button {
                        min_width: "6rem",
                        onclick: move |_evt| {
                            show.set(false);
                        },
                        "OK"
                    }

                }
            }
        }
    }
}
