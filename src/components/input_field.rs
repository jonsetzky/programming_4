use dioxus::{html::style, prelude::*};
use dioxus_logger::tracing;
use regex::Regex;

#[component]
pub fn InputField(
    label: Option<String>,
    placeholder: String,
    value: Signal<String>,
    legal_regex: Option<String>,
) -> Element {
    let re = use_hook(|| match legal_regex {
        Some(regex) => Some(Regex::new(regex.as_str()).expect("failed to build regex")),
        None => None,
    });

    rsx! {
        div {
            width: "21rem",
            display: "flex",
            align_items: "center",
            flex_direction: "column",
            justify_content: "space-between",
            gap: "2px",
            if let Some(l) = label {
                p { text_align: "left", font_size: "12px", width: "100%", "{l}" }
            }
            input {
                r#type: "text",
                oninput: move |event| {
                    let newVal = event.value();
                    let Some(re) = re.as_ref() else {
                        value.set(newVal);
                        return;
                    };
                    if newVal.is_empty() || re.is_match(&newVal) {
                        value.set(newVal);
                    }
                    else {
                        value.set(value())
                    }
                },

                value,
                placeholder,
            }

        }
    }
}
