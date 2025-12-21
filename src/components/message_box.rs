use dioxus::prelude::*;

#[component]
pub fn MessageBox(onsend: Callback<String>, disabled: bool) -> Element {
    let mut message = use_signal(|| String::from(""));

    rsx! {
        input {
            disabled,
            r#type: "text",
            value: message.read().cloned(),
            oninput: move |event| { message.set(event.value()) },
            "Message here"
        }
        button {
            disabled,
            onclick: move |_| {
                onsend.call(message.read().cloned());
                message.set(String::from(""));
            },
            "Send"
        }
    }
}
