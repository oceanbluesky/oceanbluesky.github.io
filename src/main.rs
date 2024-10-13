#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

mod components;
use components::{home::Home, blog::Blog};



#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
 

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}


fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}