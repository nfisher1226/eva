#![warn(clippy::all, clippy::pedantic)]
use clap::{App, Arg};
use std::sync::Mutex;
/// Handles getting the configuration data to and from disk
pub mod config;
/// Starts the graphical interface
mod gui;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: Mutex<config::Config> = Mutex::new(
        config::Config::from_file()
        .unwrap_or_default()
    );
}

fn main() {
    let app = App::new("eva")
        .about("A simple Gemini protocol browser")
        .author("The JenG3nie <jeang3nie@hitchhiker-linux.org>")
        .arg(Arg::new("PRIVATE")
            .help("Do not save history")
            .short('p')
            .long("private")
            .takes_value(false)
        )
        .arg(Arg::new("URI")
            .help("A uri to open")
            .takes_value(true)
            .multiple_values(true)
        );
    _ = app.get_matches();
    gui::run();
}
