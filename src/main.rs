#![warn(clippy::all, clippy::pedantic)]
#![doc = include_str!("../README.md")]
use std::sync::Mutex;
/// Everything bookmark related
pub mod bookmarks;
/// Handles getting the configuration data to and from disk
pub mod config;
/// Starts the graphical interface
mod gui;
/// Handles history creation and deletion
pub mod history;
/// Handles loading keybindings
pub mod keys;

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
    static ref SEARCH: gui::uri::Search = gui::uri::Search::load();
}

fn main() {
    gui::run();
}
