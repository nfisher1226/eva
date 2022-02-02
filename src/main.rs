#![warn(clippy::all, clippy::pedantic)]
use clap::{App, Arg};
use gtk::gio::File;
use std::sync::Mutex;
/// Handles getting the configuration data to and from disk
mod config;
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
    let matches = app.get_matches();
    if let Some(addr) = matches.values_of("URI") {
        let mut files = Vec::new();
        for uri in addr {
            files.push(String::from(uri));
        }
        gui::run(Some(files));
    } else {
        gui::run(None);
    }
}
