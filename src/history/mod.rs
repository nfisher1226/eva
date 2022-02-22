#![warn(clippy::all, clippy::pedantic)]
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

pub fn get_data_dir() -> PathBuf {
    let mut datadir = gtk::glib::user_data_dir();
    let progname = env!("CARGO_PKG_NAME");
    datadir.push(progname);
    datadir
}

pub fn get_history_file() -> PathBuf {
    let mut histfile = get_data_dir();
    histfile.push("history.toml");
    histfile
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct History {
    items: HashMap<String, String>,
}

impl History {
    pub fn append(&mut self, url: &str) {
        let now = Local::now();
        let _old = self.items.insert(String::from(url), format!("{}", now));
    }

    pub fn remove(&mut self, url: &str) {
        let _old = self.items.remove(url);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn page(&self) -> String {
        let mut page: String = String::from("# History\n");
        for (url, date) in &self.items {
            page.push_str(&format!("{}\n=> {}\n\n", date, url));
        }
        page
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let datadir = get_data_dir();
        let histfile = get_history_file();
        if !datadir.exists() {
            let dd = match datadir.to_str() {
                Some(d) => d,
                None => return Err(String::from("Empty data directory path").into()),
            };
            std::fs::create_dir(&dd)?;
        }
        let toml_string = toml::to_string(self)?;
        std::fs::write(histfile, toml_string)?;
        Ok(())
    }

    pub fn from_file() -> Result<Option<Self>, Box<dyn Error>> {
        let histfile = get_history_file();
        let histfile = if histfile.exists() {
            std::fs::read_to_string(histfile)?
        } else {
            return Ok(None);
        };
        let history = toml::from_str(&histfile)?;
        Ok(Some(history))
    }
}