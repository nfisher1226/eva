mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct Controls(ObjectSubclass<imp::Controls>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for Controls {
    fn default() -> Self {
        Self::new()
    }
}

impl Controls {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create Controls")
    }

    pub fn back_button(&self) -> gtk::Button {
        self.imp().back_button.clone()
    }

    pub fn forward_button(&self) -> gtk::Button {
        self.imp().forward_button.clone()
    }

    pub fn reload_button(&self) -> gtk::Button {
        self.imp().reload_button.clone()
    }

    pub fn addr_bar(&self) -> gtk::SearchEntry {
        self.imp().addr_bar.clone()
    }

    pub fn input_button(&self) -> gtk::MenuButton {
        self.imp().input_button.clone()
    }

    pub fn bookmark_button(&self) -> gtk::MenuButton {
        self.imp().bookmark_button.clone()
    }
}
