use dioxus::prelude::*;
use crate::components::{footer::Footer};  // Import Footer component
use crate::Route;  // Import the Route enum


#[component]
pub fn App() -> Element {
    rsx! {
        div {
            class: "app-container",
            Router::<Route> {},  // Keep the router here for navigation
            Footer {}  // Add the footer here
        }
    }
}