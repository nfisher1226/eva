#![warn(clippy::all, clippy::pedantic)]
use rgba_simple::{Color, Primary, PrimaryColor, ReducedRGBA};
use serde::{Deserialize, Serialize};

mod fonts;

use std::{fmt, fs};
use std::path::{Path, PathBuf};
use std::str::FromStr;

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
    background: Option<Color>,
    text: Color,
    link: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            background: None,
            text: Color::Reduced(ReducedRGBA::primary(PrimaryColor::Black)),
            link: Color::Reduced(ReducedRGBA::primary(PrimaryColor::Blue)),
        }
    }
}

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct Config {
    pub fonts: Fonts,
    pub colors: Colors,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fonts: Fonts::default(),
            colors: Colors::default(),
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
