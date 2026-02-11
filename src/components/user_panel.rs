use crate::components::{
    button::Button,
    tooltip::{Tooltip, TooltipContent, TooltipTrigger},
};
use dioxus::prelude::*;
use dioxus_primitives::{ContentAlign, ContentSide};

#[component]
pub fn UserPanel(username: String, connected: Signal<bool>) -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            align_items: "center",
            justify_items: "start",
            Button { class: "user-button", label: username }
            button {
                disabled: false,
                class: "logout-button",
                align_self: "end",
                flex_shrink: "1",
                flex: "0",
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
