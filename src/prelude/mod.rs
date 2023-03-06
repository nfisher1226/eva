use std::{fs, path::PathBuf};

pub use crate::{
    bookmarks::{Bookmark, BookmarkBuilder, Bookmarks},
    uri::{uri, Search},
    widgets::{
        application::Application, bookmark_editor::BookmarkEditor,
        preferences_window::PreferencesWindow, tab::Tab, theme_switcher::ThemeSwitcher,
        window::Window,
    },
};

pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = gtk::glib::user_config_dir();
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    if !configdir.exists() {
        fs::create_dir(configdir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{e}"));
    }
    configdir
}
