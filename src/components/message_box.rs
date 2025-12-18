use dioxus::prelude::*;

#[component]
pub fn MessageBox(onsend: Callback<String>) -> Element {
    let mut message = use_signal(|| String::from(""));

    rsx! {
        input {
            r#type: "text",
            value: message.read().cloned(),
            oninput: move |event| { message.set(event.value()) },
            "Message here"
        }
        button {
            onclick: move |_| {
                onsend.call(message.read().cloned());
                message.set(String::from(""));
            },
            "Send"
        }
    }
}
