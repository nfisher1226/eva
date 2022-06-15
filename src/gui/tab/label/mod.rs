mod imp;

use gtk::{
    glib::{self, Object},
    prelude::*,
    subclass::prelude::*,
};

glib::wrapper! {
    pub struct Label(ObjectSubclass<imp::Label>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for Label {
    fn default() -> Self {
        Self::new()
    }
}

impl Label {
    pub fn new() -> Self {
        Object::new(&[("orientation", &gtk::Orientation::Horizontal)])
            .expect("Cannot create tab label")
    }

    pub fn close_button(&self) -> gtk::Button {
        self.imp().button.clone()
    }

    pub fn set(&self, label: &str, spin: bool) {
        self.imp().label.set_label(label);
        if spin {
            self.imp().spinner.show();
            self.imp().spinner.start();
        } else {
            self.imp().spinner.stop();
            self.imp().spinner.hide();
        }
    }
}
