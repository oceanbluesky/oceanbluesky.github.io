// src/components/home.rs

use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "home",
            header { h1 { "Welcome to Ocean Blue Sky" } }
            img { src: "assets/images/hero.jpg", alt: "Splash Image" }
            p { "Explore my work, research, and projects." }
        }
    })
}
