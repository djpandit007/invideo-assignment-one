use dioxus::prelude::*;
use meval::eval_str;

const EVALUATOR_CSS: Asset = asset!("/assets/styling/evaluator.css");

#[component]
pub fn Evaluator() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        document::Link { rel: "stylesheet", href: EVALUATOR_CSS }
        div {
            id: "evaluator",
            h4 { "Enter a mathematical expression" }
            input {
                placeholder: "Type here to calculate...",
                oninput:  move |event| async move {
                    let data = evaluator_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Calculated value: "
                    i { "{response}" }
                }
            }
        }
    }
}


#[server(EvaluatorServer)]
async fn evaluator_server(input: String) -> Result<String, ServerFnError> {
    if input.is_empty() {
        return Ok(String::from(""));
    }
    match eval_str(&input) {
        Ok(result) => Ok(result.to_string()),
        Err(error) => Ok(String::from("Not a valid expression!")),
    }
}
