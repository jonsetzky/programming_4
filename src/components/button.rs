use dioxus::prelude::*;

#[component]
pub fn Button(
    onclick: Option<Callback>,
    label: String,
    class: Option<String>,
    disabled: Option<bool>,
) -> Element {
    rsx! {
        button {
            disabled,
            class,
            r#type: "text",
            onclick: move |_evt| {
                onclick.map(|f| f(()));
            },
            "{label}"
        }
    }
}
