use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { 
            class: "footer",
            h2 { "Footer here" }
            This is normal text to test wihtou tags
        }
    }
}