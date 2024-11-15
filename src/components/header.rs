use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { 
            class: "header mt-10", // Adds margin-top for spacing from the NavBar
            h2 { "Header here" }
        }
    }
}
