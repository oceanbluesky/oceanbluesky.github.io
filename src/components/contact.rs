use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! { 
            div { 
                class: "contact",
                h2 { "Contact" }        
            }
     }
}

// Add links to GitHub, LinkedIn, Google Scholar, email, etc.

