mod imp;

use {
    crate::BOOKMARKS,
    gtk::{
        glib::{self, Object},
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

    pub fn label(&self) -> gtk::Label {
        self.imp().label.clone()
    }

    pub fn name(&self) -> gtk::Entry {
        self.imp().name.clone()
    }

    pub fn description(&self) -> gtk::Entry {
        self.imp().description.clone()
    }

    pub fn url(&self) -> gtk::Entry {
        self.imp().url.clone()
    }

    pub fn tags(&self) -> gtk::Entry {
        self.imp().tags.clone()
    }

    pub fn update(&self, url: &str) -> bool {
        let bmarks = BOOKMARKS.lock().unwrap();
        match bmarks.all.get(url) {
            Some(b) => {
                self.imp().label.set_label("<b>Edit Bookmark</b>");
                self.imp().name.set_text(&b.name());
                self.imp().description
                    .set_text(&b.description().unwrap_or_default());
                self.imp().url.set_text(&b.url());
                self.imp().tags.set_text(&b.tags().join(" "));
                true
            }
            None => {
                self.imp().label.set_label("<b>Create Bookmark</b>");
                if let Ok(u) = Url::parse(url) {
                    self.imp().name.set_text(u.host_str().unwrap_or("Unknown host"));
                }
                self.imp().description.set_text("");
                self.imp().url.set_text(url);
                self.imp().tags.set_text("");
                false
            }
        }
    }
}
