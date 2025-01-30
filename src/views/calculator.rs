use dioxus::prelude::*;
use crate::components::Evaluator;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn Calculator() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            id: "calculator",
            Evaluator {}
        }
    }
}