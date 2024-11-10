use dioxus::prelude::*;

pub fn Post1(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Post Title 3" }
            p { "Content of the post goes here..." }
        }
    })
}
