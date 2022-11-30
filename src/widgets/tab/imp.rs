use {
    adw::gtk::{
        self,
        glib::{self, clone, subclass::{InitializingObject, Signal}},
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate,
    },
    once_cell::sync::Lazy,
    gemview::GemView,
};

#[derive(CompositeTemplate, Default)]
#[template(file = "tab.ui")]
pub struct Tab {
    #[template_child]
    pub page: TemplateChild<adw::TabPage>,
    #[template_child]
    pub back_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub forward_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub reload_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub addr_bar: TemplateChild<gtk::Entry>,
    #[template_child]
    pub bookmark_button: TemplateChild<gtk::Button>,
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
        self.connect_signals();
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
    fn connect_signals(&self) {
        let viewer = self.viewer.get();
        let page = self.page.get();
        let instance = self.instance();
        viewer.connect_page_load_started(clone!(@weak page, @weak instance => move |_,_| {
            page.set_loading(true);
            page.set_title("[loading]");
            instance.set_nav_buttons_sensitive(false);
        }));
        viewer.connect_page_load_redirect(clone!(@weak page => move |_,_| {
            page.set_loading(true);
            page.set_title("[redirect]");
        }));
        viewer.connect_page_loaded(clone!(@weak instance, @weak page => move |_,addr| {
            page.set_loading(false);
            instance.emit_by_name::<()>("page-loaded", &[&addr]);
            instance.set_nav_buttons_sensitive(true);
        }));
        viewer.connect_page_load_failed(clone!(@weak instance, @weak page => move |_,addr| {
            page.set_loading(false);
            instance.emit_by_name::<()>("page-load-failed", &[&addr]);
            instance.set_nav_buttons_sensitive(true);
        }));
        viewer.connect_request_new_tab(clone!(@weak instance => move |_,addr| {
            instance.emit_by_name::<()>("request-new-tab", &[&addr]);
        }));
        viewer.connect_request_new_window(clone!(@weak instance => move |_,addr| {
            instance.emit_by_name::<()>("request-new-tab", &[&addr]);
        }));
    }
}
