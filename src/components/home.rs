use dioxus::prelude::*;
use crate::Route; // crate: Refers to the current project or package. It’s the Rust equivalent of self when referring to the entire package/module you're working in.

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}