use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use crate::config::get_config_dir;

/// Returns the path to keys.toml
#[allow(clippy::must_use_candidate)]
pub fn get_key_file() -> PathBuf {
    let mut file = get_config_dir();
    file.push("keys.toml");
    file
}

#[derive(Clone, Debug, Deserialize)]
pub struct Keys {
    new_tab: Option<String>,
    close_tab: Option<String>,
    next_tab: Option<String>,
    prev_tab: Option<String>,
    tab1: Option<String>,
    tab2: Option<String>,
    tab3: Option<String>,
    tab4: Option<String>,
    tab5: Option<String>,
    tab6: Option<String>,
    tab7: Option<String>,
    tab8: Option<String>,
    tab9: Option<String>,
    reload: Option<String>,
    go_home: Option<String>,
    go_previous: Option<String>,
    go_next: Option<String>,
    new_window: Option<String>,
    open_bookmarks: Option<String>,
    bookmark_page: Option<String>,
    open_history: Option<String>,
    clear_history: Option<String>,
    view_source: Option<String>,
    save_page: Option<String>,
    open_prefs: Option<String>,
    open_about: Option<String>,
    quit: Option<String>,
}

impl Keys {
    pub fn from_file() -> Option<Self> {
        let keyfile = get_key_file();
        let keyfile = if keyfile.exists() {
            match fs::read_to_string(keyfile) {
                Ok(k) => k,
                Err(_) => return None,
            }
        } else {
            return None;
        };
        let keys: Self = match toml::from_str(&keyfile) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };
        Some(keys)
    }
}
