mod imp;

use adw::{
    gtk::{
        gio,
        glib::{self, Object},
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
}


