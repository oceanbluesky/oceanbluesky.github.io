// src/main.rs

#![allow(non_snake_case)]
use dioxus::prelude::*;
mod components;
use components::{
    about::About,
    contact::Contact,
    header::Header,
    home::Home,
    essays_list::EssaysList,
    essays::essay1::Essay1,
    essays::essay2::Essay2,
    essays::essay3::Essay3,
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
    #[route("/essays/essay1")]
    Essay1 {},
    #[route("/essays/essay2")]
    Essay2 {},
    #[route("/essays/essay3")]
    Essay3 {},
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