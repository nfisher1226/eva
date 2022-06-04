use {crate::BOOKMARKS, gtk::prelude::*, url::Url};

#[derive(Clone, Debug)]
pub struct BookmarkEditor {
    popover: gtk::Popover,
    label: gtk::Label,
    name: gtk::Entry,
    description: gtk::Entry,
    url: gtk::Entry,
    tags: gtk::Entry,
}

impl Default for BookmarkEditor {
    fn default() -> Self {
        let grid = gtk::builders::GridBuilder::new()
            .row_spacing(5)
            .column_spacing(5)
            .build();
        let popover = gtk::builders::PopoverBuilder::new().child(&grid).build();
        let label = gtk::builders::LabelBuilder::new()
            .use_markup(true)
            .halign(gtk::Align::Center)
            .label("<b>Create Bookmark</b>")
            .build();
        grid.attach(&label, 0, 0, 2, 1);
        let name_label = gtk::Label::new(Some("Name"));
        grid.attach(&name_label, 0, 1, 1, 1);
        let name = gtk::Entry::new();
        grid.attach(&name, 1, 1, 1, 1);
        let desc_label = gtk::Label::new(Some("Description"));
        grid.attach(&desc_label, 0, 2, 1, 1);
        let description = gtk::Entry::new();
        grid.attach(&description, 1, 2, 1, 1);
        let url_label = gtk::Label::new(Some("Url"));
        grid.attach(&url_label, 0, 3, 1, 1);
        let url = gtk::Entry::new();
        grid.attach(&url, 1, 3, 1, 1);
        let tag_label = gtk::Label::new(Some("Tags"));
        tag_label.set_valign(gtk::Align::Center);
        grid.attach(&tag_label, 0, 4, 1, 1);
        let tags = gtk::Entry::new();
        grid.attach(&tags, 1, 4, 1, 1);
        let cancel = gtk::builders::ButtonBuilder::new()
            .hexpand(false)
            .halign(gtk::Align::Start)
            .label("Cancel")
            .build();
        grid.attach(&cancel, 0, 5, 1, 1);
        let accept = gtk::builders::ButtonBuilder::new()
            .hexpand(false)
            .halign(gtk::Align::End)
            .label("Accept")
            .css_classes(vec![String::from("suggested-action")])
            .build();
        grid.attach(&accept, 1, 5, 1, 1);
        let pop = popover.clone();
        cancel.connect_clicked(move |_| pop.popdown());
        let editor = Self {
            popover,
            label,
            name,
            description,
            url,
            tags,
        };
        let ed = editor.clone();
        accept.connect_clicked(move |_| {
            let bm = (&ed).into();
            let mut bmarks = BOOKMARKS.lock().unwrap();
            bmarks.update(&bm);
            if let Err(e) = bmarks.save() {
                eprintln!("Error: {}", e);
            }
            ed.popover.popdown();
        });
        editor
    }
}

impl BookmarkEditor {
    pub fn popover(&self) -> gtk::Popover {
        self.popover.clone()
    }

    pub fn label(&self) -> gtk::Label {
        self.label.clone()
    }

    pub fn name(&self) -> gtk::Entry {
        self.name.clone()
    }

    pub fn description(&self) -> gtk::Entry {
        self.description.clone()
    }

    pub fn url(&self) -> gtk::Entry {
        self.url.clone()
    }

    pub fn tags(&self) -> gtk::Entry {
        self.tags.clone()
    }

    pub fn update(&self, url: &str) -> bool {
        let bmarks = BOOKMARKS.lock().unwrap();
        match bmarks.all.get(url) {
            Some(b) => {
                self.label.set_label("<b>Edit Bookmark</b>");
                self.name.set_text(&b.name());
                self.description.set_text(&b.description().unwrap_or_default());
                self.url.set_text(&b.url());
                self.tags.set_text(&b.tags().join(" "));
                true
            }
            None => {
                self.label.set_label("<b>Create Bookmark</b>");
                if let Ok(u) = Url::parse(url) {
                    self.name.set_text(u.host_str().unwrap_or("Unknown host"));
                }
                self.description.set_text("");
                self.url.set_text(url);
                self.tags.set_text("");
                false
            }
        }
    }
}

