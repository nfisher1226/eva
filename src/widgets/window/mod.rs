use url::Url;

mod imp;

use {
    crate::prelude::{Application, Tab},
    adw::{
        gtk::{
            gio,
            glib::{self, clone, Object},
        },
        prelude::*,
        subclass::prelude::*,
    },
};

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, adw::Window,
            gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible,
            gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
            gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)])
    }

    pub fn open_tab(&self, address: Option<&mut str>) {
        let tab = Tab::new();
        let page = self.imp().tab_view.append(&tab);
        tab.imp().connect_signals(&page);
        if let Some(addr) = address {
            tab.visit(addr);
        }
        tab.connect_page_loaded(clone!(@weak self as window, @weak page => move |_,_| {
            window.update_title(&page);
        }));
        tab.connect_page_load_failed(clone!(@weak self as window, @weak page => move |_,_| {
            window.update_title(&page);
        }));
    }

    pub fn current_tab(&self) -> Option<Tab> {
        self.imp()
            .tab_view
            .selected_page()
            .map(|x| x.child().downcast().ok())
            .flatten()
    }

    pub fn close_current_page(&self) {
        if let Some(page) = self.imp().tab_view.selected_page() {
            self.imp().tab_view.close_page(&page);
            if let Some(page) = self.imp().tab_view.selected_page() {
                self.update_title(&page);
            }
        }
    }

    fn update_title(&self, page: &adw::TabPage) {
        let tab: Tab = page.child().downcast().unwrap();
        let uri = tab.imp().viewer.uri();
        tab.imp().addr_bar.set_text(&uri);
        if let Ok(url) = Url::parse(&uri) {
            let host = url.host_str().unwrap_or_else(|| {
                if url.scheme() == "file" {
                    "filesystem"
                } else {
                    "Unknown host"
                }
            });
            self.set_title(Some(&format!(
                "{}-{} - {}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                host,
            )));
            page.set_title(host);
        }
    }
}
