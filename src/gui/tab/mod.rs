#![warn(clippy::all, clippy::pedantic)]
pub mod bookmark_editor;
pub use bookmark_editor::BookmarkEditor;

use {
    super::uri,
    crate::{BOOKMARKS, CONFIG},
    gemview::GemView,
    gtk::{glib::clone, prelude::*},
    std::{fs::File, io::{BufReader, Read}},
    url::Url,
};

#[derive(Clone, Debug)]
/// Allows persistent access to the tab label, spinner indicator and close button
pub struct Label {
    handle: gtk::Box,
    label: gtk::Label,
    spinner: gtk::Spinner,
    close_button: gtk::Button,
}

impl Default for Label {
    fn default() -> Self {
        let handle = gtk::builders::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(3)
            .build();
        let label = gtk::Label::new(Some("about:blank"));
        let spinner = gtk::Spinner::new();
        spinner.set_visible(false);
        let close_button = gtk::builders::ButtonBuilder::new()
            .icon_name("window-close-symbolic")
            .has_frame(false)
            .build();
        handle.append(&label);
        handle.append(&spinner);
        handle.append(&close_button);
        Self {
            handle,
            label,
            spinner,
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
/// A small popover for user input
pub struct Input {
    popover: gtk::Popover,
    label: gtk::Label,
    entry: gtk::Entry,
}

impl Default for Input {
    fn default() -> Self {
        let label = gtk::Label::new(None);
        let entry = gtk::Entry::new();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 3);
        vbox.append(&label);
        vbox.append(&entry);
        let popover = gtk::Popover::builder()
            .autohide(true)
            .child(&vbox)
            .has_arrow(false)
            .position(gtk::PositionType::Bottom)
            .build();
        Self {
            popover,
            label,
            entry,
        }
    }
}

impl Input {
    pub fn show(&self) {
        self.popover.popup();
    }

    pub fn request(&self, meta: &str) {
        self.label.set_label(meta);
        self.show();
    }
}

#[derive(Clone, Debug)]
pub struct Tab {
    tab: gtk::Box,
    label: Label,
    bookmark_editor: BookmarkEditor,
    input: Input,
    upload: gtk::FileChooserDialog,
    back_button: gtk::Button,
    forward_button: gtk::Button,
    reload_button: gtk::Button,
    addr_bar: gtk::SearchEntry,
    bookmark_button: gtk::MenuButton,
    viewer: GemView,
}

impl Default for Tab {
    fn default() -> Self {
        let name: String = std::iter::repeat_with(fastrand::alphanumeric)
            .take(10)
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
            .action_name("win.go_previous")
            .sensitive(false)
            .build();
        button_box.append(&back_button);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("go-next-symbolic")
            .build();
        let forward_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Go forward")
            .action_name("win.go_next")
            .sensitive(false)
            .build();
        button_box.append(&forward_button);
        let image = gtk::builders::ImageBuilder::new()
            .icon_name("view-refresh-symbolic")
            .build();
        let reload_button = gtk::builders::ButtonBuilder::new()
            .child(&image)
            .tooltip_text("Reload page")
            .action_name("win.reload")
            .sensitive(false)
            .build();
        button_box.append(&reload_button);
        let addr_bar = gtk::builders::SearchEntryBuilder::new()
            .placeholder_text("Search or enter an address")
            .hexpand(true)
            .build();
        hbox.append(&addr_bar);
        let input = Input::default();
        let upload = gtk::FileChooserDialog::builder()
            .use_header_bar(1)
            .destroy_with_parent(true)
            .modal(true)
            .title("Choose a file to upload")
            .action(gtk::FileChooserAction::Open)
            .create_folders(true)
            .build();
        upload.add_button("Accept", gtk::ResponseType::Accept);
        upload.add_button("Cancel", gtk::ResponseType::Cancel);
        let input_button = gtk::MenuButton::builder()
            .has_frame(false)
            .popover(&input.popover)
            .visible(false)
            .build();
        hbox.append(&input_button);
        let bookmark_button = gtk::builders::MenuButtonBuilder::new()
            .icon_name("bookmark-new-symbolic")
            .tooltip_text("Bookmark current page")
            .build();
        hbox.append(&bookmark_button);
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
        let bookmark_editor = BookmarkEditor::default();
        bookmark_button.set_popover(Some(&bookmark_editor.popover()));

        Self {
            tab,
            label: Label::default(),
            input,
            upload,
            bookmark_editor,
            back_button,
            forward_button,
            reload_button,
            addr_bar,
            bookmark_button,
            viewer,
        }
    }
}

impl Tab {
    pub fn init() -> Self {
        let tab = Self::default();
        tab.set_fonts();
        tab.update_bookmark_editor();
        tab.back_button.set_sensitive(false);
        tab.forward_button.set_sensitive(false);
        tab
    }

    pub fn connect_signals(&self) {
        self.addr_bar()
            .connect_activate(clone!(@strong self as tab => move |bar| {
                let mut uri = String::from(bar.text());
                uri = uri::uri(&mut uri);
                tab.viewer().visit(&uri);
            }));
        self.viewer()
            .connect_page_load_redirect(clone!(@strong self as tab => move |_, uri| {
                tab.addr_bar().set_text(&uri);
            }));
        self.viewer().connect_request_unsupported_scheme(
            clone!(@strong self as tab => move |_, uri| {
                if let Some((scheme, _)) = uri.split_once(':') {
                    match scheme {
                        "eva" => tab.request_eva_page(&uri),
                        _ => {
                            if let Err(e) = mime_open::open(&uri) {
                                eprintln!("Error opening {}: {}", uri, e);
                            }
                        }
                    }
                }
            }),
        );
        let upload = self.upload.clone();
        self.viewer().connect_request_upload(move |_viewer, _url| {
            upload.show();
        });
        self.upload.connect_response(clone!(@strong self.viewer as viewer => move |dlg,response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dlg.file() {
                    if let Some(path) = file.path() {
                        if let Ok(f) = File::open(path) {
                            let mut data: Vec<u8> = vec![];
                            let mut reader = BufReader::new(f);
                            if reader.read_to_end(&mut data).is_ok() {
                                if let Ok(url) = Url::parse(&viewer.uri()) {
                                    viewer.post_spartan(url, data);
                                }
                            }
                        }
                    }
                }
            }
            dlg.hide();
        }));
    }

    pub fn request_input(&self, meta: &str, url: String, visibility: bool) {
        let viewer = self.viewer.clone();
        let popover = self.input.popover.clone();
        self.input.entry.set_visibility(visibility);
        self.input.entry.connect_activate(move |entry| {
            let response = entry.text();
            if response.as_str() != "" {
                let mut url = url.to_string();
                url.push('?');
                let response = urlencoding::encode(response.as_str());
                url.push_str(&response);
                viewer.visit(&url);
                popover.popdown();
            }
        });
        self.input.request(meta);
    }

    pub fn tab(&self) -> gtk::Box {
        self.tab.clone()
    }

    pub fn label(&self) -> Label {
        self.label.clone()
    }

    pub fn bookmark_editor(&self) -> BookmarkEditor {
        self.bookmark_editor.clone()
    }

    /*pub fn input(&self) -> Input {
        self.input.clone()
    }*/

    pub fn upload(&self) -> gtk::FileChooserDialog {
        self.upload.clone()
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

    /*pub fn bookmark_button(&self) -> gtk::MenuButton {
        self.bookmark_button.clone()
    }*/

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

    pub fn update_bookmark_editor(&self) {
        if  self.bookmark_editor.update(self.viewer.uri().as_str()) {
            self.bookmark_button.set_icon_name("user-bookmarks-symbolic");
        } else {
            self.bookmark_button.set_icon_name("bookmark-new-symbolic");
        }
    }

    pub fn set_label(&self, label: &str, spin: bool) {
        self.label.label.set_label(label);
        if spin {
            self.label.spinner.show();
            self.label.spinner.start();
        } else {
            self.label.spinner.stop();
            self.label.spinner.hide();
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
                            self.viewer.render_gmi(&page);
                            self.viewer.set_uri(uri);
                            self.addr_bar.set_text("uri");
                            self.set_label("bookmarks", false);
                        }
                    }
                },
                //Some("history") => {}
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
        self.viewer.render_gmi(&page);
        self.viewer.set_uri("eva://bookmarks");
        self.addr_bar.set_text("eva://bookmarks");
        self.bookmark_button.set_icon_name("bookmark-new-symbolic");
        self.set_label("bookmarks", false);
    }

    fn open_bookmark_tags(&self) {
        let bookmarks = BOOKMARKS.lock().unwrap();
        let page = bookmarks.tags_to_gmi();
        self.viewer.render_gmi(&page);
        self.viewer.set_uri("eva://bookmarks/tags");
        self.addr_bar.set_text("eva://bookmarks/tags");
        self.bookmark_button.set_icon_name("bookmark-new-symbolic");
        self.set_label("bookmarks", false);
    }

    pub fn view_source(&self) {
        let mime = self.viewer.buffer_mime();
        let content = self.viewer.buffer_content();
        if mime.starts_with("text") {
            let content = String::from_utf8_lossy(&content);
            self.viewer.render_text(&content);
            self.addr_bar.set_text("eva://source");
        }
    }
}
