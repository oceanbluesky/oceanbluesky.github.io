#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{Level, info};

mod components;
use components::app::App;  // Import the App from app.rs
use components::{home::Home, blog::Blog};  // Import Home, Blog, and Footer

// Define the routes
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
    launch(App);  // Launch the App from app.rs
}

/* **To Dos:**

- [ ]  Cull screenshots of podcasts: xAI and PIML
- [ ]  Oceanbluesky,io Rust WASM
- [ ]  Read PIML
- [ ]  Nvim: AstroVim
- [ ]  Docker https://youtu.be/GFgJkfScVNU?si=1jvKC0gMuGy05EIJ
- [ ]  GitHub Actions
- [ ]  Apply xAI and QT Optimus
- [ ]  Make shift rotator operational
- [ ]  Register kindle devices
- [ ]  Ask chatGPT for help setting up Notion databases with archived page
    - [ ]  how to make linked databases for
    - [ ]  Icons for subpages
    - [ ]  Put monthly calendar in toggle
    - [ ]  Bulletin boards with pinned boards
    - [ ]  PARA  system
    - [ ]  [https://boundscraps.notion.site/boundscraps/Notion-Global-Tag-Database-Step-by-Step-303626bf3f654fc6a53b3d0654353dc3](https://www.notion.so/303626bf3f654fc6a53b3d0654353dc3?pvs=21)
    - [ ]  https://matthiasfrank.de/global-tags-in-notion/
    - [ ]  Different views for same task database depending upon location for tasks
    - [ ]  Save to Notion
    - [ ]  Formulas for Notion */