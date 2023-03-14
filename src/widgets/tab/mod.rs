mod imp;

use {
    crate::{prelude::Application, history::History, uri::uri, BOOKMARKS},
    adw::{
        gtk::glib::{self, Object},
        prelude::*,
        subclass::prelude::*,
    },
    gemview::GemView,
    url::Url,
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

    pub fn viewer(&self) -> GemView {
        self.imp().viewer.get()
    }

    pub fn add_bar(&self) -> gtk::Entry {
        self.imp().addr_bar.get()
    }

    pub fn bookmark_button(&self) -> gtk::MenuButton {
        self.imp().bookmark_button.get()
    }

    pub fn visit(&self, addr: &mut str) {
        let addr = uri(addr);
        self.imp().viewer.visit(&addr);
    }

    pub fn bind_fonts(&self, app: &Application) {
        let settings = &app.imp().settings;
        let viewer = self.imp().viewer.get();
        settings
            .bind("paragraph-font", &viewer, "font-paragraph")
            .build();
        settings.bind("quote-font", &viewer, "font-quote").build();
        settings
            .bind("preformatted-font", &viewer, "font-pre")
            .build();
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

    pub fn request_eva_page(&self, uri: &str) {
        if let Ok(url) = Url::parse(uri) {
            match url.host_str() {
                Some("bookmarks") => match url.path() {
                    "" | "/" => self.open_bookmarks(),
                    "/tags" | "/tags/" => self.open_bookmark_tags(),
                    p => {
                        let maybe_tag = p.replace("/tags/", "");
                        let bookmarks = BOOKMARKS.lock().unwrap();
                        if let Some(page) = bookmarks.tag_to_gmi(&maybe_tag) {
                            let viewer = self.viewer();
                            viewer.render_gmi(&page);
                            viewer.set_uri(uri);
                        }
                    }
                },
                Some("history") => {
                    if let Ok(Some(history)) = History::from_file() {
                        let page = history.page();
                        let viewer = self.viewer();
                        viewer.render_gmi(&page);
                        viewer.set_uri("eva://history");
                        self.add_bar().set_text("eva://history");
                        self.set_title("history");
                    }
                }
                Some("source") => {
                    self.view_source();
                }
                _ => {}
            }
        }
    }

    pub fn open_bookmarks(&self) {
        let bookmarks = BOOKMARKS.lock().unwrap();
        let page = bookmarks.to_gmi();
        let viewer = self.viewer();
        viewer.render_gmi(&page);
        viewer.set_uri("eva://bookmarks");
        self.add_bar().set_text("eva://bookmarks");
        self.bookmark_button()
            .set_icon_name("bookmark-new-symbolic");
        self.set_title("bookmarks");
    }

    fn open_bookmark_tags(&self) {
        let bookmarks = BOOKMARKS.lock().unwrap();
        let page = bookmarks.tags_to_gmi();
        let viewer = self.viewer();
        viewer.render_gmi(&page);
        viewer.set_uri("eva://bookmarks/tags");
        self.add_bar().set_text("eva://bookmarks/tags");
        self.set_title("tags");
        self.bookmark_button()
            .set_icon_name("bookmark-new-symbolic");
    }

    pub fn view_source(&self) {
        let viewer = self.viewer();
        let mime = viewer.buffer_mime();
        let content = viewer.buffer_content();
        if mime.starts_with("text") {
            let content = String::from_utf8_lossy(&content);
            viewer.render_text(&content);
            self.add_bar().set_text("eva://source");
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
