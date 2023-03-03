mod imp;

use {
    crate::{
        prelude::{Application, Tab},
        CONFIG,
    },
    adw::{
        gtk::{
            gdk::Display,
            gio,
            glib::{self, clone, Object},
            CssProvider, StyleContext,
        },
        prelude::*,
        subclass::prelude::*,
    },
    url::Url,
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
        Object::builder().property("application", app).build()
    }

    fn set_css(&self) {
        if let Ok(cfg) = CONFIG.try_lock() {
            let colors = &cfg.colors;
            let provider = CssProvider::new();
            let context = self.style_context();
            let css = include_str!("style.css")
                .replace("NORMAL_FG_COLOR", &colors.fg.to_string())
                .replace("NORMAL_BG_COLOR", &colors.bg.to_string())
                .replace("QUOTE_FG_COLOR", &colors.quote_fg.to_string())
                .replace("QUOTE_BG_COLOR", &colors.quote_bg.to_string())
                .replace("PRE_FG_COLOR", &colors.pre_fg.to_string())
                .replace("PRE_BG_COLOR", &colors.pre_bg.to_string())
                .replace("LINK_COLOR", &colors.link.to_string())
                .replace("HOVER_COLOR", &colors.hover.to_string())
                .replace("DEFAULT_FG_COLOR", &context.color().to_string())
                .replace("ReducedRGBA", "rgba")
                .replace("RGBA", "rgba");
            provider.load_from_data(&css);
            StyleContext::add_provider_for_display(
                &Display::default().expect("Cannot connect to display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    pub fn open_tab(&self, address: Option<&mut str>) {
        let tab = Tab::new();
        let app = self.application().unwrap().downcast::<crate::prelude::Application>().unwrap();
        tab.bind_fonts(&app);
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
            .and_then(|x| x.child().downcast().ok())
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

    pub fn on_reload(&self) {
        if let Some(tab) = self.current_tab() {
            tab.reload();
        }
    }

    pub fn on_go_previous(&self) {
        if let Some(tab) = self.current_tab() {
            tab.go_previous();
        }
    }

    pub fn on_go_next(&self) {
        if let Some(tab) = self.current_tab() {
            tab.go_next();
        }
    }

    pub fn on_go_home(&self) {}
}
