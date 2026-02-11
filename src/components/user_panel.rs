use crate::{
    components::{
        button::Button,
        popup::Popup,
        tooltip::{Tooltip, TooltipContent, TooltipTrigger},
    },
    route::Route,
};
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};

#[component]
pub fn UserPanel(username: String, connected: Signal<bool>) -> Element {
    let nav = navigator();
    let mut show_logout_confirmation = use_signal(|| false);
    let mut logout_button_hovered = use_signal(|| false);

    // reset hovered variable everytime logout confirmation dialog shows up
    use_effect(move || {
        if show_logout_confirmation() {
            logout_button_hovered.set(false);
        }
    });

    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_items: "start",
            Popup { show: show_logout_confirmation,
                p { font_size: "32px", "Log out?" }
                div { height: "2rem" }
                p { font_size: "18px", "Are you sure that you want to log out?" }
                div { height: "2rem" }
                p { font_size: "14px", "Logging out will erase all current messages." }
                div { flex: "1" }
                div { display: "flex", flex_direction: "row", width: "100%",
                    div { flex: "1" }

                    button {
                        id: if logout_button_hovered() { Some("final-logout-button") } else { None },
                        class: "logout-button",
                        min_width: "6rem",
                        margin: "0px",
                        onclick: move |_| {
                            nav.replace(Route::Login);
                        },
                        onmouseenter: move |_| logout_button_hovered.set(true),
                        "Yes"
                    }
                    div { width: "1rem" }
                    button {
                        min_width: "6rem",
                        onclick: move |_| show_logout_confirmation.set(false),
                        "No"
                    }
                }
            }
            Button { class: "user-button", label: username }
            button {
                disabled: false,
                class: "logout-button",
                align_self: "end",
                flex_shrink: "1",
                flex: "0",
                onclick: move |_| show_logout_confirmation.set(true),
                svg {
                    fill: "none",
                    height: "24",
                    view_box: "0 0 24 24",
                    width: "24",
                    xmlns: "http://www.w3.org/2000/svg",
                    transform: "scale(-1,1)",
                    path {
                        d: "M9 21H5C4.46957 21 3.96086 20.7893 3.58579 20.4142C3.21071 20.0391 3 19.5304 3 19V5C3 4.46957 3.21071 3.96086 3.58579 3.58579C3.96086 3.21071 4.46957 3 5 3H9M16 17L21 12M21 12L16 7M21 12H9",
                        stroke: "currentColor",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        stroke_width: "2.5",
                    }
                }
            }
            Tooltip { height: "100%",
                TooltipTrigger { height: "100%",
                    div {
                        display: "flex",
                        align_items: "center",
                        height: "100%",
                        div {
                            width: "6px",
                            height: "6px",
                            background_color: if connected() { "green" } else { "red" },
                            border_radius: "50%",
                            margin_left: "4px",
                            margin_right: "1.5rem",
                            align_self: "center",
                        }
                    }
                }
                TooltipContent {
                    side: ContentSide::Right,
                    align: ContentAlign::Center,
                    style: "background-color: #000;color: #fff",
                    p { style: "margin: 0;",
                        if connected() {
                            "Online"
                        } else {
                            "Offline"
                        }
                    }
                }
            }
        }
    }
}
