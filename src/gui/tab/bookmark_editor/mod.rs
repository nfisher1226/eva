mod imp;

use {
    crate::BOOKMARKS,
    gtk::{
        glib::{self, GString, Object},
        prelude::*,
        subclass::prelude::*,
    },
    url::Url,
};

glib::wrapper! {
    pub struct BookmarkEditor(ObjectSubclass<imp::BookmarkEditor>)
        @extends gtk::Popover, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for BookmarkEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl BookmarkEditor {
    pub fn new() -> Self {
        let editor: Self = Object::new(&[]).expect("Failed to create BookmarkEditor");
        let ed = editor.clone();
        editor.imp().cancel.connect_clicked(move |_| ed.popdown());
        let ed = editor.clone();
        editor.imp().accept.connect_clicked(move |_| {
            let bm = (&ed).into();
            let mut bmarks = BOOKMARKS.lock().unwrap();
            bmarks.update(&bm);
            if let Err(e) = bmarks.save() {
                eprintln!("Error: {}", e);
            }
            ed.popdown();
        });
        editor
    }

    pub fn name(&self) -> GString {
        self.imp().name.text()
    }

    pub fn description(&self) -> GString {
        self.imp().description.text()
    }

    pub fn url(&self) -> GString {
        self.imp().url.text()
    }

    pub fn tags(&self) -> GString {
        self.imp().tags.text()
    }

    pub fn update(&self, url: &str) -> bool {
        let bmarks = BOOKMARKS.lock().unwrap();
        let matches = bmarks.all.get(url);
        match matches {
            Some(b) => {
                self.imp().label.set_label("<b>Edit Bookmark</b>");
                self.imp().name.set_text(&b.name());
                self.imp()
                    .description
                    .set_text(&b.description().unwrap_or_default());
                self.imp().url.set_text(&b.url());
                self.imp().tags.set_text(&b.tags().join(" "));
                true
            }
            None => {
                self.imp().label.set_label("<b>Create Bookmark</b>");
                if let Ok(u) = Url::parse(url) {
                    self.imp()
                        .name
                        .set_text(u.host_str().unwrap_or("Unknown host"));
                }
                self.imp().description.set_text("");
                self.imp().url.set_text(url);
                self.imp().tags.set_text("");
                false
            }
        }
    }
}
