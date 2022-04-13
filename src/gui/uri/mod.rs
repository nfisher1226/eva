use {
    crate::{config::get_config_dir, BOOKMARKS, SEARCH},
    serde::{Deserialize, Serialize},
    std::{collections::HashMap, fs, path::PathBuf},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Search {
    pub default: String,
    pub all: HashMap<String, String>,
}

impl Default for Search {
    fn default() -> Self {
        Self {
            default: String::from("gemini://geminispace.info/search"),
            all: HashMap::from([
                (
                    String::from("gus"),
                    String::from("gemini://geminispace.info/search"),
                ),
                (
                    String::from("ron"),
                    String::from("gopher://gopher.floodgap.com/v2/vs"),
                ),
            ]),
        }
    }
}

impl Search {
    fn query(se: String, query: &[&str]) -> String {
        format!("{}?{}", se, query.join("%20"))
    }

    fn build(&self, query: &str) -> String {
        let params = query.trim().split_whitespace().collect::<Vec<&str>>();
        if let Some(se) = self.all.get(params[0]) {
            if params.len() > 1 {
                let se = se.to_string();
                Self::query(se, &params[1..])
            } else {
                let se = self.default.clone();
                Self::query(se, &params)
            }
        } else {
            let se = self.default.clone();
            Self::query(se, &params)
        }
    }

    fn save(&self) {
        let mut file = get_config_dir();
        file.push(PathBuf::from("search.toml"));
        let toml_string = toml::to_string(&self).expect("Could not encode TOML value");
        fs::write(file, toml_string).expect("Could not write to file!");
    }

    pub fn load() -> Self {
        let mut file = get_config_dir();
        file.push(PathBuf::from("search.toml"));
        if let Ok(contents) = fs::read_to_string(&file) {
            if let Ok(search) = toml::from_str(&contents) {
                return search;
            }
        }
        let search = Search::default();
        if !file.exists() {
            search.save();
        }
        search
    }
}

pub fn uri(uri: &mut str) -> Option<String> {
    if !uri.contains(':') {
        if uri.starts_with('/') {
            Some(format!("file://{}", uri))
        } else if let Some(url) = BOOKMARKS.lock().unwrap().url_from_name(&uri) {
            Some(url)
        } else if let Ok(mut path) = std::env::current_dir() {
            path = path.join(&PathBuf::from(&uri));
            if path.exists() {
                Some(format!("file://{}", path.to_string_lossy()))
            } else if uri.contains(" ") || !uri.contains(".") {
                let search = SEARCH.clone();
                Some(search.build(uri))
            } else {
                Some(format!("gemini://{}", &uri))
            }
        } else {
            let search = SEARCH.clone();
            Some(search.build(uri))
        }
    } else {
        Some(uri.to_string())
    }
}
