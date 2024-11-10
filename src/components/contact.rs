use dioxus::prelude::*;

#[allow(non_snake_case)]
pub fn Contact(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "contact",
            h2 { "Contact" }
                        }
    })
}

// Add links to GitHub, LinkedIn, Google Scholar, email, etc.

