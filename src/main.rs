#![allow(non_snake_case)]

use dioxus::prelude::*;

// Urls are relative to Cargo.toml file
const _TAILWIND_URL: &str = manganis::mg!(file("dist/assets/styles/tailwind.css"));


mod components;
use components::{
    about::About,
    contact::Contact,
    header::Header,
    home::Home, 
    essays_list::EssaysList, 
    footer::Footer,
    navbar::NavBar, 
    page_not_found::PageNotFound,
    projects::Projects,
    research::Research,
};

// ANCHOR: router
#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
    #[route("/header")]
    Header {},
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
    #[route("/contact")]
    Contact {},
    #[route("/essays_list")]
    EssaysList{},
    #[route("/footer")]
    Footer {},
    #[route("/projects")]
    Projects {},
    #[route("/research")]
    Research {},
    #[route("/:..route")]
    PageNotFound {route: Vec<String>},
}

fn main() {
    dioxus::launch(App);  // This launches the main Dioxus app
}

// ANCHOR_END: router
pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}