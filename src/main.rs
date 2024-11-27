// src/main.rs

#![allow(non_snake_case)]
use dioxus::prelude::*;

// Urls are relative to Cargo.toml file
// Disregard the error, it's a bug in the macro
const TAILWIND_URL: &str = manganis::mg!(file("dist/assets/styles/tailwind.css"));

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

#[derive(Routable, Clone, PartialEq)]
#[rustfmt::skip]
pub enum Route {
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
    dioxus::launch(App);
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}