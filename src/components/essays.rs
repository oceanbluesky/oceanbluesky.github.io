use dioxus::prelude::*;

pub fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            h1 { "Essays" }
            ul {
                li { a { href: "/post1", "Post Title 1" } }
                li { a { href: "/post2", "Post Title 2" } }
                // Add more posts as necessary
            }
        }
    })
}