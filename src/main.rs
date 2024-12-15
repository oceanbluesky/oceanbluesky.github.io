// src/main.rs

// ANY NEW ESSAY COMPONENTS AND THEIR ROUTES MUST BE ADDED TO TWO PLACES INDICATED BELOW

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
    essays::essay4::Essay4, // <<< ADD NEW ESSAY COMPONENT HERE
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
    #[route("/essays/essay4")]  // <<< ADD NEW ESSAY ROUTE HERE
    Essay4 {},
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
    dioxus::launch(app);
}

pub fn app() -> Element {
    rsx! { Router::<Route> {} }
}