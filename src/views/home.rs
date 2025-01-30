use dioxus::prelude::*;
use crate::components::Hero;

const HOME_CSS: Asset = asset!("/assets/styling/home.css");

#[component]
pub fn Home() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: HOME_CSS }
        Hero {}
        div {
            id: "home",
            p { "Hello and welcome to my assignment submission!" }
            p { "I chose to do this assignment using the Dioxus framework which is hosted via fly.io." }
            p { "This allowed me to write one and deploy across platforms. Also, I've used as the front-end and back-end language of choice for the submission." }
            p { "You can find the links to the calculator and shader tab on the top-left." }
            p { "I've implemented the evaluation functionality of the calculator using the meval crate. This allowed me to provide instant evaluations to the user." }
            p { "For the shaders part of the submission, I used Rust as my backend language instead of Elixir. I'm more familiar with the former than the latter :)" }
            p { "I've connected the input to OpenAI's gpt-4o-mini for code generation. I could not figure out a way to actually render the shaders on canvas, unfortunately." }
            p { "Finally, I'd like to say this was a fun interview assignment! Looking forward to hearing from you soon :)" }
            p { "If you have any questions or would like to connect, drop me an email at dvpandit@asu.edu or find me on LinkedIn @djpandit007" }
        }
    }
}
