mod imp;

use {
    crate::{prelude::Application, uri::uri, CONFIG},
    adw::{
        gtk::glib::{self, Object},
        prelude::*,
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct Tab(ObjectSubclass<imp::Tab>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Buildable;
}

impl Default for Tab {
    fn default() -> Self {
        Self::new()
    }
}

impl Tab {
    pub fn new() -> Self {
        Object::new()
    }

    pub fn visit(&self, addr: &mut str) {
        let addr = uri(addr);
        self.imp().viewer.visit(&addr);
    }

    pub fn bind_fonts(&self, app: &Application) {
        let settings = &app.imp().settings;
        let viewer = self.imp().viewer.get();
        settings.bind("paragraph-font", &viewer, "font-paragraph").build();
        settings.bind("quote-font", &viewer, "font-quote").build();
        settings.bind("preformatted-font", &viewer, "font-pre").build();
        settings.bind("h1-font", &viewer, "font-h1").build();
        settings.bind("h2-font", &viewer, "font-h2").build();
        settings.bind("h3-font", &viewer, "font-h3").build();
    }

    pub fn reload(&self) {
        self.imp().viewer.reload();
    }

    pub fn go_next(&self) {
        if self.imp().viewer.has_next() {
            self.imp().viewer.go_next();
        }
    }

    pub fn go_previous(&self) {
        if self.imp().viewer.has_previous() {
            self.imp().viewer.go_previous();
        }
    }

    /// Connects to the "page-loaded" signal, emitted when the internal gemview
    /// has successfully loaded a page
    pub fn connect_page_loaded<F: Fn(&Self, String) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("page-loaded", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let uri = values[1].get::<String>().unwrap();
            f(&obj, uri);
            None
        })
    }

    /// Connects to the "page-load-failed" signal, emitted whenever a page has
    /// failed to load
    pub fn connect_page_load_failed<F: Fn(&Self, String) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("page-load-failed", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let uri = values[1].get::<String>().unwrap();
            f(&obj, uri);
            None
        })
    }

    /// Connects to the "request-new-tab" signal, emitted when the "Open in new
    /// tab" item is chosen from the context menu for link items.
    pub fn connect_request_new_tab<F: Fn(&Self, String) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("request-new-tab", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let uri = values[1].get::<String>().unwrap();
            f(&obj, uri);
            None
        })
    }

    /// Connects to the "request-new-window" signal, emitted when the "Open in
    /// new window" item is chosen from the context menu for link items.
    pub fn connect_request_new_window<F: Fn(&Self, String) + 'static>(
        &self,
        f: F,
    ) -> glib::SignalHandlerId {
        self.connect_local("request-new-window", true, move |values| {
            let obj = values[0].get::<Self>().unwrap();
            let uri = values[1].get::<String>().unwrap();
            f(&obj, uri);
            None
        })
    }
}
