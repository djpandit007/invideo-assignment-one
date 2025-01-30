use crate::Route;
use dioxus::prelude::*;

const SHADER_CSS: Asset = asset!("/assets/styling/shader.css");

#[component]
pub fn Shader() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: SHADER_CSS}

        div {
            id: "shader",

            // Content
            h1 { "This is blog !" }
            p { "In blog, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }
        }
    }
}
