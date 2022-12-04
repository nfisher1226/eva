mod actions;
mod imp;

use adw::{
    gtk::{
        gio::{self, ApplicationFlags},
        glib::{self, Object},
    },
    prelude::*,
};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application, gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        Object::new(&[
            ("application-id", &Some("org.codeberg.jeang3nie.eva")),
            ("flags", &ApplicationFlags::HANDLES_OPEN),
            ("register-session", &true.to_value()),
        ])
    }

    pub fn add_actions(&self, win: &crate::prelude::Window) {
        actions::add(win, self);
    }
}
