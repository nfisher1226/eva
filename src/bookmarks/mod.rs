#![warn(clippy::all, clippy::pedantic)]
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

pub fn get_bookmarks_file() -> PathBuf {
    let mut bmarks = get_data_dir();
    bmarks.push("bookmarks.toml");
    bmarks
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Bookmark {
    name: String,
    description: Option<String>,
    url: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct BookmarkBuilder {
    name: String,
    description: Option<String>,
    url: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Bookmarks {
    /// key is url string
    pub all: HashMap<String, Bookmark>,
    /// map tag name to vec of url strings
    pub tags: HashMap<String, Vec<String>>,
}

impl BookmarkBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    pub fn description(mut self, desc: Option<&str>) -> Self {
        match desc {
            Some(d) => self.description = Some(String::from(d)),
            None => self.description = None,
        }
        self
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = String::from(url);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn build(self) -> Bookmark {
        Bookmark {
            name: self.name,
            description: self.description,
            url: self.url,
            tags: self.tags,
        }
    }
}

impl Bookmark {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn description(&self) -> Option<String> {
        match &self.description {
            Some(d) => Some(d.clone()),
            None => None,
        }
    }

    pub fn set_description(&mut self, desc: &str) {
        self.description = Some(String::from(desc));
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
    }

    pub fn tags(&self) -> Vec<String> {
        self.tags.clone()
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = tags;
    }

    pub fn append_tag(&mut self, tag: &str) {
        self.tags.push(String::from(tag));
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|x| *x != tag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }
}

impl Bookmarks {
    pub fn to_gmi(&self) -> String {
        let mut page = String::from("# Bookmarks\n\n=> eva://bookmarks/tags Tags\n\n");
        for (_, bookmark) in &self.all {
            page.push_str(&format!(
                "### Name: {}\nDescription:\n> {}\nTags: {}\n=> {}\n\n",
                &bookmark.name,
                match &bookmark.description {
                    Some(d) => &d,
                    None => "none",
                },
                &bookmark.tags.join(", "),
                &bookmark.url,
            ));
        }
        page
    }

    pub fn tags_to_gmi(&self) -> String {
        let mut page = String::from("# Bookmark Tags\n\n");
        for (tag, _) in &self.tags {
            page.push_str(&format!("=> eva://bookmarks/tags/{} {}\n", &tag, &tag));
        }
        page.push_str("--\n=> eva://bookmarks back");
        page
    }

    pub fn tag_to_gmi(&self, tag: &str) -> Option<String> {
        if let Some(keys) = self.tags.get(tag) {
            let mut page = format!("# Bookmarks tagged {}\n\n", tag);
            for key in keys {
                if let Some(bookmark) = self.all.get(key) {
                    page.push_str(&format!(
                        "### Name: {}\nDescription:\n> {}\nTags: {}\n=> {}\n\n",
                        &bookmark.name,
                        match &bookmark.description {
                            Some(d) => &d,
                            None => "none",
                        },
                        &bookmark.tags.join(", "),
                        &bookmark.url,
                    ));
                }
            }
            page.push_str("--\n=> eva://bookmarks/tags back");
            Some(page)
        } else {
            None
        }
    }

    pub fn update(&mut self, bookmark: &Bookmark) {
        self.all.insert(bookmark.url.clone(), bookmark.clone());
        for tag in &bookmark.tags {
            match self.tags.get(tag) {
                Some(t) => {
                    let mut v = t.clone();
                    v.push(bookmark.url.clone());
                    self.tags.insert(tag.to_string(), v);
                }
                None => {
                    _ = self
                        .tags
                        .insert(tag.to_string(), vec![bookmark.url.clone()])
                }
            }
        }
        for (tag, urls) in &self.tags.clone() {
            if urls.is_empty() {
                let _t = self.tags.remove(tag);
            } else {
                let mut u = urls.clone();
                u.sort();
                u.dedup();
                if !bookmark.has_tag(&tag) {
                    u.retain(|x| x != &bookmark.url);
                }
                self.tags.insert(tag.to_string(), u);
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let datadir = get_data_dir();
        let bmfile = get_bookmarks_file();
        if !datadir.exists() {
            let dd = match datadir.to_str() {
                Some(d) => d,
                None => return Err(String::from("Empty data directory path").into()),
            };
            std::fs::create_dir(&dd)?;
        }
        let toml_string = toml::to_string(self)?;
        std::fs::write(bmfile, toml_string)?;
        Ok(())
    }

    pub fn from_file() -> Result<Option<Self>, Box<dyn Error>> {
        let bmarks = get_bookmarks_file();
        let bmarks = if bmarks.exists() {
            std::fs::read_to_string(bmarks)?
        } else {
            return Ok(None);
        };
        let bookmarks = toml::from_str(&bmarks)?;
        Ok(Some(bookmarks))
    }

    pub fn url_from_name(&self, name: &str) -> Option<String> {
        for (_, bookmark) in &self.all {
            if bookmark.name().as_str() == name {
                return Some(bookmark.url());
            }
        }
        None
    }
}
