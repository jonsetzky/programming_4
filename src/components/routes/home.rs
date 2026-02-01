use std::{io, time::Duration};

use dioxus::{html::h1, prelude::*};
use dioxus_primitives::{ContentAlign, ContentSide};
use tokio::select;

use crate::{
    AppState,
    components::{Button, tooltip::Tooltip, tooltip::TooltipContent, tooltip::TooltipTrigger},
    route::Route,
    tcp_chat_client::TcpChatClient,
};

async fn read_loop(mut client: TcpChatClient) {
    loop {
        let packet = match client.recv().await {
            Err(err) => {
                if err.kind() == io::ErrorKind::ConnectionAborted {
                    break;
                } else {
                    println!("unknown error while attempting to recv()");
                    break;
                }
            }
            Ok(packet) => packet,
        };
        println!(
            "got packet {}",
            String::from_utf8(packet.into_bytes()).unwrap()
        );
    }
}
#[component]
pub fn Home() -> Element {
    let nav = navigator();
    let state = use_context::<AppState>();
    let mut connection_notification = state.connection_notification;
    let mut connected = use_signal(|| false);

    use_future(move || async move {
        loop {
            connected.set(false);
            let client = match TcpChatClient::connect(Some(state.address.to_string().as_str()))
                .await
            {
                Ok(client) => client,
                Err(err) => {
                    connection_notification.set(String::from("Error connecting to the server."));
                    // println!("error connecting. attempting again in 5 seconds");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    connected.set(false);
                    continue;
                }
            };

            connected.set(true);
            connection_notification.set(String::from(""));

            let _read_handle = tokio::spawn(async move {
                read_loop(client.clone()).await;
            });

            let _ = _read_handle.await;
        }
    });

    rsx! {
        // p {
        //     onclick: move |_| {
        //         nav.replace(Route::Login);
        //     },
        //     "home"
        // }
        svg {
            width: "360",
            height: "198",
            view_box: "0 0 360 198",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            style: "position: absolute; left: 333px; top: 87px;",

            path {
                d: "M358.81 196.442C358.201 194.607 355.748 190.928 351.603 186.469C347.896 182.481 344.682 178.939 339.467 174.328C332.452 168.126 325.629 162.635 322.701 159.721C319.449 156.484 315.485 153.727 311.792 150.499C307.601 146.836 298.923 140.226 291.665 134.061C286.873 129.991 274.983 122.427 266.305 116.839C259.986 112.769 256.465 110.07 252.006 107.004C247.78 104.097 244.477 101.77 240.783 98.8562C237.229 96.0516 231.584 93.1755 225.885 89.7911C216.87 84.4371 211.296 82.1092 206.662 80.0989C197.567 76.153 183.612 69.4937 178.084 66.7317C171.992 63.6885 166.091 62.1161 161.327 60.1196C152.918 56.5948 142.114 53.2078 132.458 49.3715C118.562 43.8503 110.759 40.2971 107.066 38.7525C103.474 37.2503 99.0842 36.2948 94.3118 34.2983C89.3833 32.2364 83.0934 30.7617 77.8692 29.3784C73.3904 28.1925 68.6519 27.0729 64.4928 25.0764C60.0851 22.9605 54.8191 23.3842 50.66 22.153C46.2203 20.8389 41.2997 20.304 36.9793 19.3865C32.6746 18.4722 27.762 17.851 23.9165 16.7721C19.5961 15.5599 15.7735 15.0844 3.8081 14.6187C1.90834 14.5448 2.82597 14.1623 3.75277 13.8533C7.44614 12.6222 11.1257 11.3957 14.8191 9.24698C18.6559 7.01483 22.192 4.33171 24.9724 2.63488C25.3661 2.39457 24.0732 1.56514 23.1464 1.25159C19.6367 0.0642201 15.7735 3.39107 12.0801 4.77436C8.00286 6.30143 5.6202 8.92421 4.08476 10.3029C3.348 10.9644 3.15796 11.9859 3.45767 12.7605C4.96918 16.6672 12.0479 15.3795 15.5891 16.915C19.3475 18.5446 20.3476 23.9744 23.2755 27.5156C23.9715 28.3573 24.9585 28.8989 24.9724 29.3646C24.9862 29.8303 24.0732 30.1346 23.1464 30.1392C19.3408 30.1581 15.7735 27.9951 12.2323 25.6942C9.30896 22.7663 6.23807 20.9311 4.08937 18.6303C3.15796 17.5467 2.54931 16.6337 1.00003 14.7709",
                stroke: "#404040",
                stroke_width: "2",
                stroke_linecap: "round",
            }
        }
        div {
            display: "flex",
            flex_direction: "row",
            width: "100vw",
            height: "100vh",
            justify_items: "start",
            div {
                display: "flex",
                flex_direction: "column",
                height: "100vh",
                width: "20rem",
                background_color: "#262626",
                flex_shrink: "0",
                // align_items: "center",
                gap: "4px",
                h2 {
                    onclick: move |_| {
                        nav.replace(Route::Login);
                    },
                    padding: "1rem",
                    padding_top: "1.2rem",
                    "Your Neighborhoods"
                }
                hr { align_self: "center" }
                Button { class: "neighborhood-button", label: "Kauppakatu 213" }
                Button {
                    class: "neighborhood-button",
                    label: "Naapurusto, jolla on aivan liian pitkä nimi, mikä ei meinaa loppua koskaan",
                }
                hr { align_self: "center" }
                Button { class: "add-neighborhood-button", label: "+ Add" }
                div { flex: "1" }
                hr { align_self: "center" }
                div {
                    display: "flex",
                    flex_direction: "row",
                    align_items: "center",
                    justify_items: "start",
                    Button { class: "user-button", label: state.username }
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
            div {
                display: "flex",
                width: "100%",
                height: "100%",
                flex_grow: "0",
                justify_items: "center",
                align_items: "center",
                div {
                    display: "flex",
                    flex_direction: "column",
                    margin_left: "auto",
                    margin_right: "auto",
                    width: "14rem",
                    h1 { font_size: "32px", color: "#606060", "Get Started" }
                    p {
                        font_size: "16px",
                        font_weight: "900",
                        color: "#404040",
                        "by selecting a neighborhood on the left"
                    }
                }
            }
        }
    }
}
