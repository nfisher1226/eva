#![warn(clippy::all, clippy::pedantic)]
//! Contents
//! ========
//! * [Introduction](#introduction)
//! * [Features](#features)
//! * [Keybindings](#keybindings)
//! * [Building](#building)
//! ## Introduction
//! Eva is a [gemini protocol](https://gemini.circumlunar.space/) browser written in
//! [Rust](https://rust-lang.org) using the [gtk+](https://gtk-rs.org/) toolkit. Eva
//! is currently alpha quality software under heavy development and as such is
//! missing features and may have bugs.
//! ## Features
//! - [x] tabbed interface
//! - [ ] bookmarks
//! - [x] keyboard shortcuts
//!   - [ ] user configurable
//! - [ ] user controlled styling
//!   - [x] fonts
//!   - [ ] colors
//! - [x] back-forward list
//! - [ ] history
//!
//! ## Keybindings
//! | Key | Action |
//! | --- | --- |
//! | Ctrl/T | New tab |
//! | Ctrl/N | New window |
//! | Ctrl/W | Close tab |
//! | Ctrl/Q | Close window |
//! | Ctrl/R | Reload page |
//! | Alt/Home | Go to homepage |
//! | Ctrl/PageDown | Next tab |
//! | Ctrl/PageUp | Previous tab |
//! | Alt/[1-9] | nth tab |
//! | Ctrl/Shift/O | Open bookmarks |
//! | Ctrl/D | Bookmark page |
//! | Ctrl/H | Open History |
//! | Ctrl/Shift/P | Open preferences |
//! | Ctrl/Shift/A | Open about dialog |
//!
//! ## Building
//! ```sh
//! # clone the source
//! git clone https://codeberg.org/jeang3nie/eva.git
//! cd eva
//! cargo run -- gemini://gemini.circumlunar.space
//! ```
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
