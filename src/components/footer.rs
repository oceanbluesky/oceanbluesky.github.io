// src/components/home.rs
use dioxus::prelude::*;
use std::thread::Scope;

#[allow(non_snake_case)]
pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Footer here" }
    })
}