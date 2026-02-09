use crate::components::{
    Button,
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
                            margin_left: "4px", // Add some space between the button and the circle
                            margin_right: "1.5rem",
                            align_self: "center",
                        }
                    }
                }
                TooltipContent {
                    // The side of the TooltipTrigger where the content will be displayed. Can be one of Top, Right, Bottom, or Left.
                    side: ContentSide::Right,
                    // The alignment of the TooltipContent relative to the TooltipTrigger. Can be one of Start, Center, or End.
                    align: ContentAlign::Center,
                    style: "background-color: #000;color: #fff",
                    // The content of the tooltip, which can include text, images, or any other elements.
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
