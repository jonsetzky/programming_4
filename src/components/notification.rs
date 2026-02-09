use dioxus::prelude::*;

use crate::AppState;

#[component]
pub fn Notification() -> Element {
    let state = use_context::<AppState>();
    let notification = state.connection_notification;

    let class = if notification.is_empty() {
        "notification hide"
    } else {
        "notification"
    };

    rsx! {
        div { class, "{notification}" }
    }
}
