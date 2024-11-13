use dioxus::prelude::*;

#[component]
pub fn Research() -> Element {
    rsx! {
        div { class: "research",
            h2 { "Research" }
            p { "Research projects and interests." }
        }
    }
}