use dioxus::prelude::*;

pub fn Post1(cx: ScopeState) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Post Title 2" }
            p { "Content of the post goes here..." }
        }
    })
}
