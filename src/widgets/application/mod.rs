mod actions;
mod imp;

use adw::gtk::{
    gio,
    glib::{self, Object},
};

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends adw::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}

impl Application {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn add_actions(&self, win: &crate::prelude::Window) {
        actions::add(win, self);
    }
}
