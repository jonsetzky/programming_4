use dioxus::{html::h1, prelude::*};

use crate::route::Route;

#[component]
pub fn Home() -> Element {
    let nav = navigator();
    rsx! {
        p {
            onclick: move |_| {
                nav.replace(Route::Login);
            },
            "home"
        }
    }
}
