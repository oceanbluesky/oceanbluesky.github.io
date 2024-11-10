use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Research(cx: ScopeState) -> Element {
    cx.render(rsx! {
        div { class: "research",
            h2 { "Research" }
            p { "Research projects and interests." }
        }
    })
}
