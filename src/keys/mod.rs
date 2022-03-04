use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::config::get_config_dir;

/// Returns the path to keys.toml
#[allow(clippy::must_use_candidate)]
pub fn get_key_file() -> PathBuf {
    let mut file = get_config_dir();
    file.push("keys.toml");
    file
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Keys {
    keys: HashMap<String, String>
}

impl Keys {
    pub fn new_tab(&self) -> &str {
        if let Some(key) = self.keys.get("new_tab") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>T"
    }

    pub fn close_tab(&self) -> &str {
        if let Some(key) = self.keys.get("close_tab") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>W"
    }

    pub fn next_tab(&self) -> &str {
        if let Some(key) = self.keys.get("next_tab") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>Page_Down"
    }

    pub fn prev_tab(&self) -> &str {
        if let Some(key) = self.keys.get("prev_tab") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>Page_Up"
    }

    pub fn tab1(&self) -> &str {
        if let Some(key) = self.keys.get("tab1") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>1"
    }

    pub fn tab2(&self) -> &str {
        if let Some(key) = self.keys.get("tab2") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>2"
    }

    pub fn tab3(&self) -> &str {
        if let Some(key) = self.keys.get("tab3") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>3"
    }

    pub fn tab4(&self) -> &str {
        if let Some(key) = self.keys.get("tab4") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>4"
    }

    pub fn tab5(&self) -> &str {
        if let Some(key) = self.keys.get("tab5") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>5"
    }

    pub fn tab6(&self) -> &str {
        if let Some(key) = self.keys.get("tab6") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>6"
    }

    pub fn tab7(&self) -> &str {
        if let Some(key) = self.keys.get("tab7") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>7"
    }

    pub fn tab8(&self) -> &str {
        if let Some(key) = self.keys.get("tab8") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>8"
    }

    pub fn tab9(&self) -> &str {
        if let Some(key) = self.keys.get("tab9") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>9"
    }

    pub fn reload(&self) -> &str {
        if let Some(key) = self.keys.get("reload") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>R"
    }

    pub fn go_home(&self) -> &str {
        if let Some(key) = self.keys.get("go_home") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>Home"
    }

    pub fn go_previous(&self) -> &str {
        if let Some(key) = self.keys.get("go_previous") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>Left"
    }

    pub fn go_next(&self) -> &str {
        if let Some(key) = self.keys.get("go_next") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<Alt>Right"
    }

    pub fn new_window(&self) -> &str {
        if let Some(key) = self.keys.get("new_window") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>N"
    }

    pub fn open_bookmarks(&self) -> &str {
        if let Some(key) = self.keys.get("open_bookmarks") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary><Shift>O"
    }

    pub fn bookmark_page(&self) -> &str {
        if let Some(key) = self.keys.get("bookmark_page") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>D"
    }

    pub fn open_history(&self) -> &str {
        if let Some(key) = self.keys.get("open_history") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>H"
    }

    pub fn view_source(&self) -> &str {
        if let Some(key) = self.keys.get("view_source") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>U"
    }

    pub fn save_page(&self) -> &str {
        if let Some(key) = self.keys.get("save_page") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>S"
    }

    pub fn open_prefs(&self) -> &str {
        if let Some(key) = self.keys.get("open_prefs") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary><Shift>P"
    }

    pub fn open_about(&self) -> &str {
        if let Some(key) = self.keys.get("open_about") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary><Shift>A"
    }

    pub fn quit(&self) -> &str {
        if let Some(key) = self.keys.get("quit") {
            if gtk::accelerator_parse(key).is_some() {
                return key;
            }
        }
        "<primary>Q"
    }

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
