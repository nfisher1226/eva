#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]

use adw::prelude::*;

fn main() {
    //eva::gui::run();
    let app = eva::prelude::Application::new();
    app.run();
}
