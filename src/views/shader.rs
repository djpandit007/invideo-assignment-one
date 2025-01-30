use crate::Route;
use dioxus::prelude::*;

const SHADER_CSS: Asset = asset!("/assets/styling/shader.css");

#[component]
pub fn Shader() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SHADER_CSS }

        div {
            id: "shader",
            h4 { "Enter a mathematical expression" }
            input {
                placeholder: "Type here to calculate...",
                oninput:  move |event| async move {},
            }
            button { id: "generate", "Generate"}

        }
    }
}
