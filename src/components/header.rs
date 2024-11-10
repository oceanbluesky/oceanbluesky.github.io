// src/components/home.rs
use dioxus::prelude::*;
use std::thread::Scope;


#[allow(non_snake_case)]
pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div { "Header here" }
    })
}