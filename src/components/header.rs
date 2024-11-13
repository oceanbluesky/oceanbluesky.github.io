use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { 
            class: "header",
            h2 { "Header here" }
        }
    }
}