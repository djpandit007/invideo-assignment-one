use crate::Route;
use dioxus::prelude::*;

const SHADER_CSS: Asset = asset!("/assets/styling/shader.css");

#[component]
pub fn Shader() -> Element {
    let mut text_prompt = use_signal(|| String::new());
    let mut shader_response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: SHADER_CSS }

        div {
            id: "shader",
            h4 { "Text to Shader" }
            input {
                placeholder: "A rotating cube with a gradient background",
                oninput:  move |event| text_prompt.set(event.value()),
            }
            button {
                id: "generate",
                onclick: move |_| async move {
                    let value = crate::backend::call_openai(text_prompt.to_string()).await;
                    shader_response.set(value.unwrap());
                },
                "Generate code"
            }

            div {
                pre { "{shader_response}" }
            }
        }
    }
}
