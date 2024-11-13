use dioxus::prelude::*;

#[component]
pub fn Essay1(name: String) -> Element {
    rsx! { h2 { "Essay1: {name}" } }
}
