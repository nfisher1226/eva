#![warn(clippy::all, clippy::pedantic)]
//! Contents
//! ========
//! * [Introduction](#introduction)
//! * [Building](#building)
//! * [Features](#features)
//! ## Introduction
//! Eva is a [gemini protocol](https://gemini.circumlunar.space/) browser written in
//! [Rust](https://rust-lang.org) using the [gtk+](https://gtk-rs.org/) toolkit. Eva
//! is currently alpha quality software under heavy development and as such is
//! missing features and may have bugs.
//!
//! ## Building
//! ```sh
//! # clone the source
//! git clone https://codeberg.org/jeang3nie/eva.git
//! cd eva
//! cargo run -- gemini://gemini.circumlunar.space
//! ```
//! ## Features
//! - [x] tabbed interface
//! - [ ] bookmarks
//! - [x] keyboard shortcuts
//!   - [ ] user configurable
//! - [ ] user controlled styling
//! - [x] back-forward list
//! - [ ] history
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
