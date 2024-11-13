use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! { 
        h1 { "Welcome to the Dioxus Blog!" } 
    }
}
