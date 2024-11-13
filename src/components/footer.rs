use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { 
            class: "footer",
            h2 { "Footer here" }
        }
    }
}