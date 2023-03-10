#![warn(clippy::all, clippy::pedantic)]

use {
    chrono::prelude::*,
    gtk::glib,
    nix::fcntl::{self, FlockArg},
    serde::{Deserialize, Serialize},
    std::{
        collections::HashMap,
        error::Error,
        fmt::Write,
        fs::{self, File},
        io::{self, Read},
        os::{fd::AsRawFd, unix::prelude::FileExt},
        path::PathBuf,
        thread,
    },
};

#[must_use]
pub fn get_data_dir() -> PathBuf {
    let mut datadir = glib::user_data_dir();
    let progname = env!("CARGO_PKG_NAME");
    datadir.push(progname);
    datadir
}

#[must_use]
pub fn get_history_file() -> PathBuf {
    let mut histfile = get_data_dir();
    histfile.push("history.toml");
    histfile
}

pub fn append_history(url: &str) -> Result<(), io::Error> {
    let url = url.to_string();
    thread::spawn(move || {
        if !get_data_dir().exists() {
            fs::create_dir_all(get_data_dir()).expect("Couldn't create data directory");
        }
        let histfile = get_history_file();
        if !histfile.exists() {
            let mut history = History::default();
            history.append(&url);
            history.save().expect("Couldn't save history");
        } else {
            let mut fd = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .append(false)
                .open(&histfile)
                .expect("cannot open histfile");
            fcntl::flock(fd.as_raw_fd(), FlockArg::LockExclusive).expect("cannot lock histfile");
            let mut history = History::from_fd(&mut fd).expect("Cannot get history");
            history.append(&url);
            history.write(&mut fd).expect("Cannot save history");
        }
    });
    Ok(())
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct History {
    items: HashMap<String, chrono::DateTime<Local>>,
}

impl History {
    pub fn append(&mut self, url: &str) {
        let _old = self.items.insert(String::from(url), Local::now());
    }

    pub fn remove(&mut self, url: &str) {
        let _old = self.items.remove(url);
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    #[must_use]
    pub fn page(&self) -> String {
        let mut page: String = String::from("# History\n");
        for (url, date) in &self.items {
            let _ = write!(page, "{date}\n=> {url}\n\n");
        }
        page
    }

    /// # Errors
    /// Returns an error if unable to get the data directory path, unable to
    /// create the data directory, unable to serialize toml or unable to write
    /// the toml to disk
    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let datadir = get_data_dir();
        let histfile = get_history_file();
        if !datadir.exists() {
            let Some(dd) = datadir.to_str() else {
                return Err(String::from("Empty data directory path").into());
            };
            fs::create_dir(dd)?;
        }
        let toml_string = toml::to_string(self)?;
        fs::write(histfile, toml_string)?;
        Ok(())
    }

    fn write(&self, fd: &mut File) -> Result<(), Box<dyn Error>> {
        let toml_string = toml::to_string(self)?;
        fd.write_all_at(&toml_string.as_bytes(), 0)?;
        Ok(())
    }

    /// # Errors
    /// Returns an error if unable to read the history from disk, or unable to
    /// deserialize toml
    pub fn from_file() -> Result<Option<Self>, Box<dyn Error>> {
        let histfile = get_history_file();
        let histfile = if histfile.exists() {
            fs::read_to_string(histfile)?
        } else {
            return Ok(None);
        };
        let history = toml::from_str(&histfile)?;
        Ok(Some(history))
    }

    fn from_fd(fd: &mut File) -> Result<Self, Box<dyn Error>> {
        let mut history = String::new();
        fd.read_to_string(&mut history)?;
        let history = toml::from_str(&history)?;
        Ok(history)
    }
}
