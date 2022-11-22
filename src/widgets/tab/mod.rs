mod imp;

use {
    crate::uri::uri,
    adw::{
        gtk::{
            gio,
            glib::{self, Object},
        },
        prelude::*,
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct Tab(ObjectSubclass<imp::Tab>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl Tab {
    pub fn new() -> Self {
        Object::new(&[])
    }

    pub fn visit(&self, addr: &mut str) {
        let addr = uri(addr);
        self.imp().viewer.visit(&addr);
    }
}
