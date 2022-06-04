use gtk::prelude::*;

#[derive(Clone, Debug)]
/// A small popover for user input
pub struct Input {
    popover: gtk::Popover,
    label: gtk::Label,
    entry: gtk::Entry,
}

impl Default for Input {
    fn default() -> Self {
        let label = gtk::Label::new(None);
        let entry = gtk::Entry::new();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 3);
        vbox.append(&label);
        vbox.append(&entry);
        let popover = gtk::Popover::builder()
            .autohide(true)
            .child(&vbox)
            .has_arrow(false)
            .position(gtk::PositionType::Bottom)
            .build();
        Self {
            popover,
            label,
            entry,
        }
    }
}

impl Input {
    pub fn popover(&self) -> gtk::Popover {
        self.popover.clone()
    }

    pub fn entry(&self) -> gtk::Entry {
        self.entry.clone()
    }

    pub fn set_visibility(&self, visibility: bool) {
        self.entry.set_visibility(visibility);
    }

    pub fn show(&self) {
        self.popover.popup();
    }

    pub fn request(&self, meta: &str) {
        self.label.set_label(meta);
        self.show();
    }
}
