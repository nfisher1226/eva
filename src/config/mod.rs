#![warn(clippy::all, clippy::pedantic)]
use rgba_simple::{Color, Primary, PrimaryColor, ReducedRGBA};
use serde::{Deserialize, Serialize};

mod fonts;

use std::fs;
use std::path::{Path, PathBuf};

pub use fonts::{Font, FontStyle, Fonts};

/// Returns an OS appropriate configuration directory path
pub fn get_config_dir() -> PathBuf {
    let mut configdir: PathBuf = gtk::glib::user_config_dir();
    let progname = env!("CARGO_PKG_NAME");
    configdir.push(progname);
    if !configdir.exists() {
        fs::create_dir(&configdir.to_str().unwrap()).unwrap_or_else(|e| eprintln!("{}", e));
    }
    configdir
}

/// Returns the path to config.toml
pub fn get_config_file() -> PathBuf {
    let mut file = get_config_dir();
    file.push("config.toml");
    file
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Colors {
    pub fg: Color,
    pub bg: Color,
    pub quote_fg: Color,
    pub quote_bg: Color,
    pub link: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            fg: Color::Reduced(ReducedRGBA{ red: 24, green: 24, blue: 24, alpha: 255 }),
            bg: Color::Reduced(ReducedRGBA{ red: 200, green: 200, blue: 200, alpha: 255 }),
            quote_fg: Color::Reduced(ReducedRGBA{ red: 24, green: 24, blue: 24, alpha: 255 }),
            quote_bg: Color::Reduced(ReducedRGBA{ red: 210, green: 175, blue: 95, alpha: 255 }),
            link: Color::Reduced(ReducedRGBA::primary(PrimaryColor::Blue)),
        }
    }
}

impl Colors {
    pub fn fg(&self) -> Color {
        self.fg.clone()
    }

    pub fn set_fg(&mut self, color: Color) {
        self.fg = color;
    }

    pub fn bg(&self) -> Color {
        self.bg.clone()
    }

    pub fn set_bg(&mut self, color: Color) {
        self.bg = color;
    }

    pub fn quote_fg(&self) -> Color {
        self.quote_fg.clone()
    }

    pub fn set_quote_fg(&mut self, color: Color) {
        self.quote_fg = color;
    }

    pub fn quote_bg(&self) -> Color {
        self.quote_bg.clone()
    }

    pub fn set_quote_bg(&mut self, color: Color) {
        self.quote_bg = color;
    }

    pub fn link(&self) -> Color {
        self.link.clone()
    }

    pub fn set_link(&mut self, color: Color) {
        self.link = color;
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum NewPage {
    Home,
    Blank,
}

impl Default for NewPage {
    fn default() -> Self {
        Self::Home
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum ShowTabs {
    Always,
    Multiple,
    Never,
}

impl Default for ShowTabs {
    fn default() -> Self {
        Self::Always
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub enum TabPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl Default for TabPosition {
    fn default() -> Self {
        Self::Top
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct General {
    pub homepage: String,
    pub new_page: NewPage,
    pub show_tabs: ShowTabs,
    pub tab_position: TabPosition,
}

impl Default for General {
    fn default() -> Self {
        Self {
            homepage: String::from("gemini://gemini.circumlunar.space/"),
            new_page: NewPage::default(),
            show_tabs: ShowTabs::default(),
            tab_position: TabPosition::default(),
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    pub general: General,
    pub colors: Colors,
    pub fonts: Fonts,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General::default(),
            colors: Colors::default(),
            fonts: Fonts::default(),
        }
    }
}

impl Config {
    /// Saves Config struct as a .toml file
    pub fn save_to_file(&self, file: &Path) {
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
        fs::write(file.clone(), toml_string).expect("Could not write to file!");
    }

    /// Deserializes config.toml into a `GfretConfig` struct
    pub fn from_file() -> Option<Self> {
        let config_file = get_config_file();
        let config_file = if config_file.exists() {
            match fs::read_to_string(config_file) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        } else {
            return None;
        };
        let config: Self = match toml::from_str(&config_file) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("{}", e);
                return None;
            }
        };
        Some(config)
    }
}
