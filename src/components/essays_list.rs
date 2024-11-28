
// src/components/essays_list.rs

use dioxus::prelude::*;

#[component]
pub fn EssaysList() -> Element {
    rsx! {
        div {
            class: "essays ml-64 mt-16 mb-3",
            h2 {
                class: "font-display font-bold text-5xl text-black",
                "Essays"
            }
            p { 
                class: "font-text text-lg text-gray-500 mb-4", 
                "A brief description of philosophy shared here." 
            }
            ul {
                class: "font-text text-lg text-gray-500", 
                li { a { href: "/essays/essay1", "Essay Title 1" } }
                li { a { href: "/essays/essay2", "Essay Title 2" } }
                li { a { href: "/essays/essay3", "Essay Title 3" } }
            }
        }
    }
}

