use gtk::prelude::*;
use gemview::GemView;

#[derive(Clone, Debug)]
pub struct TabLabel {
    handle: gtk::Box,
    label: gtk::Label,
    close_button: gtk::Button,
}

impl Default for TabLabel {
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

impl TabLabel {
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
    label: TabLabel,
    back_button: gtk::Button,
    forward_button: gtk::Button,
    reload_button: gtk::Button,
    addr_bar: gtk::SearchEntry,
    viewer: GemView,
}

impl Default for Tab {
    fn default() -> Self {
        let tab = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
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
        let bbox = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .homogeneous(true)
            .css_classes(vec![String::from("linked")])
            .margin_end(15)
            .build();
        hbox.append(&bbox);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("go-previous-symbolic")
            .build();
        let back_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Go back")
            .sensitive(false)
            .build();
        bbox.append(&back_button);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("go-next-symbolic")
            .build();
        let forward_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Go forward")
            .sensitive(false)
            .build();
        bbox.append(&forward_button);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("view-refresh-symbolic")
            .margin_start(6)
            .margin_end(6)
            .build();
        let reload_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Reload page")
            .sensitive(false)
            .build();
        bbox.append(&reload_button);
        let addr_bar = gtk::builders::SearchEntryBuilder::new()
            .placeholder_text("Search or enter an address")
            .hexpand(true)
            .build();
        hbox.append(&addr_bar);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("bookmark-new-symbolic")
            .margin_start(6)
            .margin_end(6)
            .build();
        let scroller = gtk::builders::ScrolledWindowBuilder::new()
            .hexpand(true)
            .vexpand(true)
            .build();
        let viewer = GemView::new();
        viewer.set_margin_start(25);
        viewer.set_margin_end(25);
        viewer.set_margin_top(25);
        viewer.set_margin_bottom(25);
        scroller.set_child(Some(&viewer));
        tab.append(&scroller);

        Self {
            tab,
            label: TabLabel::default(),
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

    pub fn label(&self) -> TabLabel {
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
}
