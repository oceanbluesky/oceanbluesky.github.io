use dioxus::prelude::*;

#[component]
pub fn EssaysList() -> Element {
    rsx! {
        div {
            class: "essays",
            h1 { "Essays" }
            ul {
                li { a { href: "/essays/essay1", "Essay Title 1" } }
                li { a { href: "/essays/essay2", "Essay Title 2" } }
                li { a { href: "/essays/essay3", "Essay Title 3" } }
            }
        }
    }
}

