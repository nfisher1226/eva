mod imp;

use {
    crate::{uri::uri, CONFIG},
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
        Object::new(&[])
    }

    pub fn visit(&self, addr: &mut str) {
        let addr = uri(addr);
        self.imp().viewer.visit(&addr);
    }

    fn set_nav_buttons_sensitive(&self, sensitive: bool) {
        self.imp().reload_button.set_sensitive(sensitive);
        if sensitive {
            let back = self.imp().viewer.has_previous();
            self.imp().back_button.set_sensitive(back);
            let next = self.imp().viewer.has_next();
            self.imp().back_button.set_sensitive(next);
        } else {
            self.imp().forward_button.set_sensitive(sensitive);
            self.imp().back_button.set_sensitive(sensitive);
        }
    }

    pub fn set_fonts(&self) {
        if let Ok(cfg) = CONFIG.try_lock() {
            let fonts = cfg.fonts.clone();
            self.imp().viewer.set_font_paragraph(fonts.pg.to_pango());
            self.imp().viewer.set_font_quote(fonts.quote.to_pango());
            self.imp().viewer.set_font_pre(fonts.pre.to_pango());
            self.imp().viewer.set_font_h1(fonts.h1.to_pango());
            self.imp().viewer.set_font_h2(fonts.h2.to_pango());
            self.imp().viewer.set_font_h3(fonts.h3.to_pango());
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
