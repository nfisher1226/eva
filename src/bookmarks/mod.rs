#![warn(clippy::all, clippy::pedantic)]
use {
    crate::gui::tab::BookmarkEditor,
    gtk::prelude::*,
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, error::Error, fmt::Write, path::PathBuf},
};

#[must_use]
pub fn get_data_dir() -> PathBuf {
    let mut datadir = gtk::glib::user_data_dir();
    let progname = env!("CARGO_PKG_NAME");
    datadir.push(progname);
    datadir
}

#[must_use]
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
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn name(mut self, name: &str) -> Self {
        self.name = String::from(name);
        self
    }

    #[must_use]
    pub fn description(mut self, desc: Option<&str>) -> Self {
        match desc {
            Some(d) => self.description = Some(String::from(d)),
            None => self.description = None,
        }
        self
    }

    #[must_use]
    pub fn url(mut self, url: &str) -> Self {
        self.url = String::from(url);
        self
    }

    #[must_use]
    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    #[must_use]
    pub fn build(self) -> Bookmark {
        Bookmark {
            name: self.name,
            description: self.description,
            url: self.url,
            tags: self.tags,
        }
    }
}

impl From<&BookmarkEditor> for Bookmark {
    fn from(editor: &BookmarkEditor) -> Self {
        BookmarkBuilder::new()
            .name(editor.name().text().as_str())
            .description(match editor.description().text().as_str() {
                "" => None,
                s => Some(s),
            })
            .url(editor.url().text().as_str())
            .tags(
                editor.tags()
                    .text()
                    .to_string()
                    .split_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect(),
            )
            .build()
    }
}

impl Bookmark {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    #[must_use]
    pub fn description(&self) -> Option<String> {
        self.description.as_ref().cloned()
    }

    pub fn set_description(&mut self, desc: &str) {
        self.description = Some(String::from(desc));
    }

    #[must_use]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn set_url(&mut self, url: &str) {
        self.url = String::from(url);
    }

    #[must_use]
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

    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&String::from(tag))
    }
}

impl Bookmarks {
    #[must_use]
    pub fn to_gmi(&self) -> String {
        let mut page = String::from("# Bookmarks\n\n=> eva://bookmarks/tags Tags\n\n");
        for bookmark in self.all.values() {
            let _ = writeln!(
                page,
                "### Name: {}\nDescription:\n> {}\nTags: {}\n=> {}\n",
                &bookmark.name,
                match &bookmark.description {
                    Some(d) => d,
                    None => "none",
                },
                &bookmark.tags.join(", "),
                &bookmark.url,
            );
        }
        page
    }

    #[must_use]
    pub fn tags_to_gmi(&self) -> String {
        let mut page = String::from("# Bookmark Tags\n\n");
        for tag in self.tags.keys() {
            let _ = writeln!(page, "=> eva://bookmarks/tags/{} {}", &tag, &tag);
        }
        page.push_str("--\n=> eva://bookmarks back");
        page
    }

    #[must_use]
    pub fn tag_to_gmi(&self, tag: &str) -> Option<String> {
        if let Some(keys) = self.tags.get(tag) {
            let mut page = format!("# Bookmarks tagged {}\n\n", tag);
            for key in keys {
                if let Some(bookmark) = self.all.get(key) {
                    let _ = write!(
                        page,
                        "### Name: {}\nDescription:\n> {}\nTags: {}\n=> {}\n\n",
                        &bookmark.name,
                        match &bookmark.description {
                            Some(d) => d,
                            None => "none",
                        },
                        &bookmark.tags.join(", "),
                        &bookmark.url,
                    );
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
                    let _in = self
                        .tags
                        .insert(tag.to_string(), vec![bookmark.url.clone()]);
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
                if !bookmark.has_tag(tag) {
                    u.retain(|x| x != &bookmark.url);
                }
                self.tags.insert(tag.to_string(), u);
            }
        }
    }

    /// # Errors
    /// Returns error if unable to serialize toml or write to file
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

    /// # Errors
    /// Returns error if unable to read bookmarks file or deserialize toml
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

    #[must_use]
    pub fn url_from_name(&self, name: &str) -> Option<String> {
        for bookmark in self.all.values() {
            if bookmark.name().as_str() == name {
                return Some(bookmark.url());
            }
        }
        None
    }
}
