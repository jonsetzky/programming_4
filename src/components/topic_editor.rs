use dioxus::prelude::*;

use crate::AppState;

#[component]
pub fn TopicEditor(topic: Signal<String>) -> Element {
    let mut show_topic_editor = use_signal(|| false);
    let mut new_topic = use_signal(String::default);

    use_effect(move || {
        // toggling topic editor visibility resets its value
        let _ = show_topic_editor();
        new_topic.set(String::default());
    });

    rsx! {
        div {
            display: "flex",
            justify_content: "center",
            align_items: "center",
            width: "100%",
            height: "1.5rem",
            vertical_align: "middle",
            background_color: "#1d1d1d",
            button {
                r#type: "text",
                text_align: "center",
                class: "low-profile",
                color: "#ddd",
                flex: "1",
                font_size: "14px",
                font_weight: "200",
                padding: "0px 4px 0px 4px", // trbl
                border_radius: "0px",
                r#type: "text",
                autofocus: true,
                // lower level than oninput, which allows preventing the modification of the text
                onclick: move |_evt| {
                    show_topic_editor.toggle();
                },
                {topic}
            }
            if show_topic_editor() {
                div {
                    position: "absolute",
                    top: "2rem",
                    margin_top: "0.2rem",
                    display: "flex",
                    flex_direction: "row",
                    width: "24rem",
                    height: "2rem",
                    flex_grow: "1",
                    justify_items: "center",
                    align_items: "center",
                    input {
                        class: "topic-editor",
                        r#type: "text",
                        autofocus: true,
                        flex: "1",
                        height: "100%",
                        font_size: "14px",
                        font_weight: "200",
                        oninput: move |evt| {
                            new_topic.set(evt.value());
                        },
                        placeholder: "A new interesting topic",
                        value: new_topic(),
                    }
                    button {
                        flex_grow: "0",
                        flex_shrink: "0",
                        width: "2.3rem",
                        height: "100%",
                        min_width: "16px",
                        border_radius: "0px 6px 6px 0px",

                        display: "flex",
                        align_items: "center",
                        justify_content: "center",

                        onclick: move |_| {
                            spawn(async move {
                                let state = consume_context::<AppState>();
                                let packet_sender = (state.packet_sender)();
                                match packet_sender {
                                    Some(packet_sender) => {
                                        let _ = packet_sender
                                            .send(state.packet_builder().set_topic(new_topic()))
                                            .await;

                                        show_topic_editor.set(false);
                                    }
                                    None => {
                                        println!("cant send message because packet_sender is null");
                                    }
                                }

                            });
                        },

                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 1920 1920",
                            width: "18",
                            height: "18",
                            fill: "#ddd",
                            stroke: "#ddd",
                            stroke_width: "100",

                            path {
                                fill_rule: "evenodd",
                                d: "M1827.701 303.065 698.835 1431.801 92.299 825.266 0 917.564 698.835 1616.4 1919.869 395.234z",
                            }
                        }
                    }
                }
            }
        }
    }
}
