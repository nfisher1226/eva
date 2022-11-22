/// Everything bookmark related
pub mod bookmarks;
/// Handles getting the configuration data to and from disk
pub mod config;
/// Starts the graphical interface
pub mod gui;
/// Handles history creation and deletion
pub mod history;
/// Handles loading keybindings
pub mod keys;
pub mod prelude;
pub mod uri;
pub mod widgets;

use {lazy_static::lazy_static, std::sync::Mutex};

lazy_static! {
    static ref CONFIG: Mutex<config::Config> =
        Mutex::new(config::Config::from_file().unwrap_or_default());
    static ref BOOKMARKS: Mutex<bookmarks::Bookmarks> =
        Mutex::new(match bookmarks::Bookmarks::from_file() {
            Ok(b) => b.unwrap_or_default(),
            Err(_) => bookmarks::Bookmarks::default(),
        });
    static ref SEARCH: uri::Search = uri::Search::load();
}
