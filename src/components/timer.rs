use std::time::SystemTime;

use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[component]
pub fn Timer() -> Element {
    let mut time = use_signal(SystemTime::now);

    use_effect(move || {
        spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                *time.write() = SystemTime::now();
            }
        });
    });

    let datetime: DateTime<Utc> = (*time.read()).into();
    let time_str = datetime.to_rfc3339_opts(chrono::SecondsFormat::Millis, false);

    rsx!(
        p { "{time_str}" }
    )
}
