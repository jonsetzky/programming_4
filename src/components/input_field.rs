use dioxus::prelude::*;
use regex::Regex;

#[component]
pub fn InputField(
    class: Option<String>,
    label: Option<String>,
    placeholder: String,
    value: Signal<String>,
    legal_regex: Option<String>,
    onblur: Option<Callback<Event<FocusData>>>,
    onillegal: Option<EventHandler>,
) -> Element {
    let re = use_hook(|| {
        legal_regex.map(|regex| Regex::new(regex.as_str()).expect("failed to build regex"))
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
                style: "width: 100%",
                // lower level than oninput, which allows preventing the modification of the text
                // this could be used instead
                // onkeydown: move |evt| {
                // },
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
                        if let Some(onillegal) = onillegal {
                            onillegal(());
                        }
                        value.set(value())
                    }
                },
                onblur: move |evt| {
                    let Some(onblur) = onblur else {
                        return;
                    };
                    onblur(evt);
                },
                value,
                placeholder,
            }

        }
    }
}
