use gemview::GemView;
use gtk::prelude::*;

use crate::CONFIG;

#[derive(Clone, Debug)]
pub struct Label {
    handle: gtk::Box,
    label: gtk::Label,
    close_button: gtk::Button,
}

impl Default for Label {
    fn default() -> Self {
        let handle = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(3)
            .build();
        let label = gtk::Label::new(Some("about:blank"));
        let close_button = gtk::builders::ButtonBuilder::new()
            .icon_name("window-close-symbolic")
            .has_frame(false)
            .build();
        handle.append(&label);
        handle.append(&close_button);
        Self {
            handle,
            label,
            close_button,
        }
    }
}

impl Label {
    pub fn handle(&self) -> gtk::Box {
        self.handle.clone()
    }

    pub fn label(&self) -> gtk::Label {
        self.label.clone()
    }

    pub fn close_button(&self) -> gtk::Button {
        self.close_button.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Tab {
    tab: gtk::Box,
    label: Label,
    back_button: gtk::Button,
    forward_button: gtk::Button,
    reload_button: gtk::Button,
    addr_bar: gtk::SearchEntry,
    viewer: GemView,
}

impl Default for Tab {
    fn default() -> Self {
        let name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(15)
            .collect();
        let tab = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .name(&name)
            .build();
        let hbox = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(3)
            .margin_start(3)
            .margin_end(3)
            .margin_top(3)
            .margin_bottom(3)
            .build();
        tab.append(&hbox);
        let button_box = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .css_classes(vec![String::from("linked")])
            .margin_end(15)
            .build();
        hbox.append(&button_box);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("go-previous-symbolic")
            .build();
        let back_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Go back")
            .sensitive(false)
            .build();
        button_box.append(&back_button);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("go-next-symbolic")
            .build();
        let forward_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Go forward")
            .sensitive(false)
            .build();
        button_box.append(&forward_button);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("view-refresh-symbolic")
            .margin_start(6)
            .margin_end(6)
            .build();
        let reload_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Reload page")
            .sensitive(false)
            .name("reload_button")
            .build();
        button_box.append(&reload_button);
        let addr_bar = gtk::builders::SearchEntryBuilder::new()
            .placeholder_text("Search or enter an address")
            .hexpand(true)
            .build();
        hbox.append(&addr_bar);
        let scroller = gtk::builders::ScrolledWindowBuilder::new()
            .hexpand(true)
            .vexpand(true)
            .propagate_natural_width(true)
            .css_classes(vec!["gemview".to_string()])
            .build();
        let viewer = GemView::new();
        viewer.set_margin_start(25);
        viewer.set_margin_end(25);
        viewer.set_margin_top(25);
        viewer.set_margin_bottom(25);
        viewer.set_css_classes(&["gemview"]);
        scroller.set_child(Some(&viewer));
        tab.append(&scroller);

        Self {
            tab,
            label: Label::default(),
            back_button,
            forward_button,
            reload_button,
            addr_bar,
            viewer,
        }
    }
}

impl Tab {
    pub fn tab(&self) -> gtk::Box {
        self.tab.clone()
    }

    pub fn label(&self) -> Label {
        self.label.clone()
    }

    pub fn back_button(&self) -> gtk::Button {
        self.back_button.clone()
    }

    pub fn forward_button(&self) -> gtk::Button {
        self.forward_button.clone()
    }

    pub fn reload_button(&self) -> gtk::Button {
        self.reload_button.clone()
    }

    pub fn addr_bar(&self) -> gtk::SearchEntry {
        self.addr_bar.clone()
    }

    pub fn viewer(&self) -> GemView {
        self.viewer.clone()
    }

    pub fn set_fonts(&self) {
        let cfg = CONFIG.lock().unwrap().clone();
        self.viewer.set_font_paragraph(cfg.fonts.pg.to_pango());
        self.viewer.set_font_quote(cfg.fonts.quote.to_pango());
        self.viewer.set_font_pre(cfg.fonts.pre.to_pango());
        self.viewer.set_font_h1(cfg.fonts.h1.to_pango());
        self.viewer.set_font_h2(cfg.fonts.h2.to_pango());
        self.viewer.set_font_h3(cfg.fonts.h3.to_pango());
    }
}
