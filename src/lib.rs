/// Everything bookmark related
mod bookmarks;
/// Handles getting the configuration data to and from disk
mod config;
/// Starts the graphical interface
pub mod gui;
/// Handles history creation and deletion
pub mod history;
/// Handles loading keybindings
pub mod keys;
mod uri;
mod widgets;

use {lazy_static::lazy_static, std::sync::Mutex};

pub use {
    bookmarks::{Bookmark, BookmarkBuilder, Bookmarks},
    config::{get_config_dir, get_config_file, Config},
    uri::{uri, Search},
    widgets::{
        application::Application, bookmark_editor::BookmarkEditor, tab::Tab, window::Window,
    },
};

lazy_static! {
    static ref CONFIG: Mutex<Config> = Mutex::new(Config::from_file().unwrap_or_default());
    static ref BOOKMARKS: Mutex<Bookmarks> = Mutex::new(match Bookmarks::from_file() {
        Ok(b) => b.unwrap_or_default(),
        Err(_) => Bookmarks::default(),
    });
    static ref SEARCH: Search = Search::load();
}
