use dioxus::prelude::*;

#[component]
fn App() -> Element {
    rsx! { "Hello, world!" }
}

fn main() {
    dioxus::launch(App);
}
