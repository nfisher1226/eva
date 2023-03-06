/// Everything bookmark related
pub mod bookmarks;
/// Handles history creation and deletion
pub mod history;
/// Handles loading keybindings
pub mod keys;
pub mod prelude;
pub mod uri;
pub mod widgets;

use {lazy_static::lazy_static, std::sync::Mutex};

lazy_static! {
    static ref BOOKMARKS: Mutex<bookmarks::Bookmarks> =
        Mutex::new(match bookmarks::Bookmarks::from_file() {
            Ok(b) => b.unwrap_or_default(),
            Err(_) => bookmarks::Bookmarks::default(),
        });
    static ref SEARCH: uri::Search = uri::Search::load();
}
