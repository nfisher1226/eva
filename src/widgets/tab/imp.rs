use {
    crate::prelude::BookmarkEditor,
    adw::gtk::{
        self,
        glib::{
            self, clone,
            subclass::{InitializingObject, Signal},
        },
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    gemview::GemView,
    once_cell::sync::Lazy,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "tab.ui")]
pub struct Tab {
    #[template_child]
    pub back_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub forward_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub reload_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub addr_bar: TemplateChild<gtk::Entry>,
    #[template_child]
    pub bookmark_button: TemplateChild<gtk::MenuButton>,
    #[template_child]
    pub bookmark_editor: TemplateChild<BookmarkEditor>,
    #[template_child]
    pub scroller: TemplateChild<gtk::ScrolledWindow>,
    #[template_child]
    pub viewer: TemplateChild<GemView>,
}

#[glib::object_subclass]
impl ObjectSubclass for Tab {
    const NAME: &'static str = "Tab";
    type Type = super::Tab;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for Tab {
    fn constructed(&self) {
        self.parent_constructed();
        self.instance().set_fonts();
    }

    fn signals() -> &'static [Signal] {
        static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
            vec![
                Signal::builder("page-loaded")
                    .param_types([glib::Type::STRING])
                    .build(),
                Signal::builder("page-load-failed")
                    .param_types([glib::Type::STRING])
                    .build(),
                Signal::builder("request-new-tab")
                    .param_types([glib::Type::STRING])
                    .build(),
                Signal::builder("request-new-window")
                    .param_types([glib::Type::STRING])
                    .build(),
            ]
        });
        SIGNALS.as_ref()
    }
}

impl BoxImpl for Tab {}
impl WidgetImpl for Tab {}

impl Tab {
    pub fn connect_signals(&self, page: &adw::TabPage) {
        let viewer = self.viewer.get();
        let instance = self.instance();
        viewer.connect_page_load_started(clone!(@weak page, @weak self as s => move |_,_| {
            s.on_page_load_started(&page);
        }));
        viewer.connect_page_load_redirect(clone!(@weak page => move |_,_| {
            Self::on_page_redirect(&page);
        }));
        viewer.connect_page_loaded(
            clone!(@weak instance, @weak page, @weak self as s => move |_,addr| {
                s.on_page_loaded(&page, &addr);
            }),
        );
        viewer.connect_page_load_failed(
            clone!(@weak instance, @weak page, @weak self as s => move |_,addr| {
                s.on_page_load_failed(&page, &addr);
            }),
        );
        viewer.connect_request_new_tab(clone!(@weak instance => move |_,addr| {
            instance.emit_by_name::<()>("request-new-tab", &[&addr]);
        }));
        viewer.connect_request_new_window(clone!(@weak instance => move |_,addr| {
            instance.emit_by_name::<()>("request-new-tab", &[&addr]);
        }));
        self.addr_bar
            .get()
            .connect_activate(clone!(@weak self as s => move |_| {
                s.on_addr_bar_activate();
            }));
    }

    fn on_page_load_started(&self, page: &adw::TabPage) {
        page.set_loading(true);
        page.set_title("[loading]");
        self.set_nav_buttons_sensitive(false);
    }

    fn on_page_redirect(page: &adw::TabPage) {
        page.set_loading(true);
        page.set_title("[redirect]");
    }

    fn on_page_loaded(&self, page: &adw::TabPage, addr: &str) {
        page.set_loading(false);
        self.instance().emit_by_name::<()>("page-loaded", &[&addr]);
        self.set_nav_buttons_sensitive(true);
        self.update_bookmark_editor();
    }

    fn on_page_load_failed(&self, page: &adw::TabPage, addr: &str) {
        page.set_loading(false);
        self.instance()
            .emit_by_name::<()>("page-load-failed", &[&addr]);
        self.set_nav_buttons_sensitive(true);
    }

    fn on_addr_bar_activate(&self) {
        let mut uri = String::from(self.addr_bar.get().text());
        uri = crate::uri::uri(&mut uri);
        self.viewer.get().visit(&uri);
    }

    fn set_nav_buttons_sensitive(&self, sensitive: bool) {
        self.reload_button.get().set_sensitive(sensitive);
        let back_button = self.back_button.get();
        let forward_button = self.forward_button.get();
        let viewer = self.viewer.get();
        if sensitive {
            back_button.set_sensitive(viewer.has_previous());
            forward_button.set_sensitive(viewer.has_next());
        } else {
            forward_button.set_sensitive(false);
            back_button.set_sensitive(false);
        }
    }

    pub fn update_bookmark_editor(&self) {
        if self
            .bookmark_editor
            .get()
            .update(self.viewer.uri().as_str())
        {
            self.bookmark_button
                .get()
                .set_icon_name("user-bookmarks-symbolic");
        } else {
            self.bookmark_button
                .get()
                .set_icon_name("bookmark-new-symbolic");
        }
    }
}
