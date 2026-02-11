use dioxus::prelude::*;

#[component]
pub fn Popup(
    children: Element,
    show: Signal<bool>,
    /// Whether clicking the background closes the popup. True by default.
    background_closes: Option<bool>,
) -> Element {
    rsx! {
        if show() {
            div {
                z_index: "2",
                position: "fixed",
                height: "100vh",
                width: "100vw",
                top: "50%",
                left: "50%",
                transform: "translate(-50%, -50%)",
                background_color: "rgba(0, 0, 0, 0.32)",
                display: "flex",
                justify_content: "center",
                align_items: "center",
                onclick: move |_| {
                    if background_closes.unwrap_or(true) {
                        show.set(false);
                    }
                },
                div {
                    background_color: "#262626",
                    width: "24rem",
                    height: "16rem",
                    display: "flex",
                    flex_direction: "column",
                    padding: "2rem",
                    onclick: move |evt| {
                        if background_closes.unwrap_or(true) {
                            evt.stop_propagation();
                        }
                    },
                    {children}
                }
            }
        }
    }
}
