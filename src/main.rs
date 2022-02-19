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
//! is focused on delivering a simple yet powerful interface and clear, readable
//! and beautiful page rendering. It has no designs on being the most fully featured
//! Gemini browser, but instead wants to be the most polished native Gtk+ Gemini
//! client that stays out of the user's way.
//!
//! Eva's gemtext rendering has been designed to be clean yet visually appealing,
//! using modern css styling to visually separate elements such as block quotes and
//! preformatted sections, using colored cards with rounded edges and box shadows.
//! The default color scheme has been chosen to provide great readability, while
//! giving the user a great deal of control over appearance.
//!
//! Eva is currently alpha quality software and is under heavy development. Some
//! features are not yet implemented and there may be bugs. However, rest assured
//! that when Eva sees an official release it will be a polished user experience.
//! ## Features
//! - [x] tabbed interface
//! - [x] bookmarks
//! - [x] keyboard shortcuts
//!   - [ ] user configurable
//! - [x] user controlled styling
//!   - [x] user specified fonts
//!   - [x] user specified colors
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
//! | Alt/Left | Go back |
//! | Alt/Right | Go forward |
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
/// Everything bookmark related
pub mod bookmarks;
/// Handles getting the configuration data to and from disk
pub mod config;
/// Starts the graphical interface
mod gui;
/// Handles history creation and deletion
pub mod history;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CONFIG: Mutex<config::Config> =
        Mutex::new(config::Config::from_file().unwrap_or_default());
    static ref BOOKMARKS: Mutex<bookmarks::Bookmarks> =
        Mutex::new(match bookmarks::Bookmarks::from_file() {
            Ok(b) => b.unwrap_or_default(),
            Err(_) => bookmarks::Bookmarks::default(),
        });
}

fn main() {
    let app = App::new("eva")
        .about("A simple Gemini protocol browser")
        .author("The JenG3nie <jeang3nie@hitchhiker-linux.org>")
        .arg(
            Arg::new("PRIVATE")
                .help("Do not save history")
                .short('p')
                .long("private")
                .takes_value(false),
        )
        .arg(
            Arg::new("URI")
                .help("A uri to open")
                .takes_value(true)
                .multiple_values(true),
        );
    let _matches = app.get_matches();
    gui::run();
}
