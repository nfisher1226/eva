#![warn(clippy::all, clippy::pedantic)]

mod dialogs;
mod tab;
pub mod uri;
use {
    crate::{config, keys::Keys, CONFIG},
    dialogs::Dialogs,
    gemview::GemView,
    gtk::{
        gdk::Display,
        gio::{Cancellable, Notification, SimpleAction},
        glib,
        glib::{char::Char, clone, OptionArg, OptionFlags},
        prelude::*,
        Application, CssProvider, ResponseType, StyleContext,
    },
    mime2ext::mime2ext,
    std::{borrow::Cow, cell::RefCell, collections::HashMap, fs, path::PathBuf, rc::Rc},
    tab::Tab,
    url::Url,
};

struct Actions {
    new_tab: SimpleAction,
    close_tab: SimpleAction,
    next_tab: SimpleAction,
    prev_tab: SimpleAction,
    tab1: SimpleAction,
    tab2: SimpleAction,
    tab3: SimpleAction,
    tab4: SimpleAction,
    tab5: SimpleAction,
    tab6: SimpleAction,
    tab7: SimpleAction,
    tab8: SimpleAction,
    tab9: SimpleAction,
    reload: SimpleAction,
    go_home: SimpleAction,
    go_previous: SimpleAction,
    go_next: SimpleAction,
    new_window: SimpleAction,
    open_bookmarks: SimpleAction,
    bookmark_page: SimpleAction,
    open_history: SimpleAction,
    clear_history: SimpleAction,
    view_source: SimpleAction,
    save_page: SimpleAction,
    open_prefs: SimpleAction,
    open_about: SimpleAction,
    quit: SimpleAction,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            new_tab: SimpleAction::new("new_tab", None),
            close_tab: SimpleAction::new("close_tab", None),
            next_tab: SimpleAction::new("next_tab", None),
            prev_tab: SimpleAction::new("prev_tab", None),
            tab1: SimpleAction::new("tab1", None),
            tab2: SimpleAction::new("tab2", None),
            tab3: SimpleAction::new("tab3", None),
            tab4: SimpleAction::new("tab4", None),
            tab5: SimpleAction::new("tab5", None),
            tab6: SimpleAction::new("tab6", None),
            tab7: SimpleAction::new("tab7", None),
            tab8: SimpleAction::new("tab8", None),
            tab9: SimpleAction::new("tab9", None),
            reload: SimpleAction::new("reload", None),
            go_home: SimpleAction::new("go_home", None),
            go_previous: SimpleAction::new("go_previous", None),
            go_next: SimpleAction::new("go_next", None),
            new_window: SimpleAction::new("new_window", None),
            open_bookmarks: SimpleAction::new("open_bookmarks", None),
            bookmark_page: SimpleAction::new("bookmark_page", None),
            open_history: SimpleAction::new("open_history", None),
            clear_history: SimpleAction::new("clear_history", None),
            view_source: SimpleAction::new("view_source", None),
            save_page: SimpleAction::new("save_page", None),
            open_prefs: SimpleAction::new("open_prefs", None),
            open_about: SimpleAction::new("open_about", None),
            quit: SimpleAction::new("quit", None),
        }
    }
}

impl Actions {
    fn connect_tab(&self, gui: &Rc<Gui>) {
        self.new_tab
            .connect_activate(clone!(@strong gui => move |_, _| {
                gui.new_tab(None);
            }));

        self.close_tab
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.close_current_tab();
            }));

        self.next_tab
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.next_tab();
            }));

        self.prev_tab
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.prev_tab();
            }));

        self.tab1.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(0);
        }));

        self.tab2.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(1);
        }));

        self.tab3.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(2);
        }));

        self.tab4.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(3);
        }));

        self.tab5.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(4);
        }));

        self.tab6.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(5);
        }));

        self.tab7.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(6);
        }));

        self.tab8.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(7);
        }));

        self.tab9.connect_activate(clone!(@weak gui => move |_,_| {
            gui.notebook.set_page(8);
        }));
    }

    fn connect_nav(&self, gui: &Rc<Gui>) {
        self.reload
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.reload_current_tab() {
                    eprintln!("{}", e);
                }
            }));

        self.go_home
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.go_home() {
                    eprintln!("{}", e);
                }
            }));

        self.go_previous
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.go_previous() {
                    eprintln!("{}", e);
                }
            }));

        self.go_next
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Err(e) = gui.go_next() {
                    eprintln!("{}", e);
                }
            }));
    }

    fn connect(&self, gui: &Rc<Gui>, app: &Application) {
        self.connect_tab(gui);
        self.connect_nav(gui);
        self.new_window
            .connect_activate(clone!(@weak gui, @strong app => move |_,_| {
                let new_gui = build_ui(&app);
                new_gui.new_tab(None);
            }));

        self.open_bookmarks
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.open_bookmarks();
            }));

        self.bookmark_page
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Some(tab) = gui.current_tab() {
                    tab.bookmark_editor().popover().popup();
                }
            }));

        self.open_history
            .connect_activate(clone!(@weak gui => move |_,_| {
                println!("Not implemented yet");
            }));

        self.view_source
            .connect_activate(clone!(@weak gui => move |_,_| {
                if let Some(tab) = gui.current_tab() {
                    tab.view_source();
                }
            }));

        self.save_page
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.save_page();
            }));

        self.clear_history
            .connect_activate(clone!(@weak gui => move |_,_| {
                println!("Not implemented yet");
            }));

        self.open_prefs
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.dialogs.preferences.show();
            }));

        self.open_about
            .connect_activate(clone!(@weak gui => move |_,_| {
                gui.dialogs.about.show();
            }));

        self.quit.connect_activate(clone!(@weak gui => move |_,_| {
            gui.window.close();
        }));
    }
}

#[derive(Clone)]
struct Gui {
    window: gtk::ApplicationWindow,
    notebook: gtk::Notebook,
    tabs: RefCell<HashMap<String, Tab>>,
    dialogs: Dialogs,
}

impl Default for Gui {
    fn default() -> Self {
        let builder = gtk::Builder::from_string(include_str!("main.ui"));
        let window: gtk::ApplicationWindow = builder.object("mainWindow").unwrap();
        let notebook: gtk::Notebook = builder.object("mainNotebook").unwrap();
        let tabs: RefCell<HashMap<String, Tab>> = RefCell::new(HashMap::new());
        let dialogs: Dialogs = Dialogs::init(&window, &builder);

        Self {
            window,
            notebook,
            tabs,
            dialogs,
        }
    }
}

impl Gui {
    fn add_actions(&self, app: &gtk::Application) -> Actions {
        let actions = Actions::default();
        let keys = Keys::from_file().unwrap_or_default();

        app.set_accels_for_action("win.new_tab", &[keys.new_tab()]);
        app.set_accels_for_action("win.close_tab", &[keys.close_tab()]);
        app.set_accels_for_action("win.next_tab", &[keys.next_tab()]);
        app.set_accels_for_action("win.prev_tab", &[keys.prev_tab()]);
        app.set_accels_for_action("win.tab1", &[keys.tab1()]);
        app.set_accels_for_action("win.tab2", &[keys.tab2()]);
        app.set_accels_for_action("win.tab3", &[keys.tab3()]);
        app.set_accels_for_action("win.tab4", &[keys.tab4()]);
        app.set_accels_for_action("win.tab5", &[keys.tab5()]);
        app.set_accels_for_action("win.tab6", &[keys.tab6()]);
        app.set_accels_for_action("win.tab7", &[keys.tab7()]);
        app.set_accels_for_action("win.tab8", &[keys.tab8()]);
        app.set_accels_for_action("win.tab9", &[keys.tab9()]);
        app.set_accels_for_action("win.reload", &[keys.reload()]);
        app.set_accels_for_action("win.go_home", &[keys.go_home()]);
        app.set_accels_for_action("win.go_previous", &[keys.go_previous()]);
        app.set_accels_for_action("win.go_next", &[keys.go_next()]);
        app.set_accels_for_action("win.new_window", &[keys.new_window()]);
        app.set_accels_for_action("win.open_bookmarks", &[keys.open_bookmarks()]);
        app.set_accels_for_action("win.bookmark_page", &[keys.bookmark_page()]);
        app.set_accels_for_action("win.open_history", &[keys.open_history()]);
        app.set_accels_for_action("win.view_source", &[keys.view_source()]);
        app.set_accels_for_action("win.save_page", &[keys.save_page()]);
        app.set_accels_for_action("win.open_prefs", &[keys.open_prefs()]);
        app.set_accels_for_action("win.open_about", &[keys.open_about()]);
        app.set_accels_for_action("win.quit", &[keys.quit()]);

        self.window.add_action(&actions.new_tab);
        self.window.add_action(&actions.close_tab);
        self.window.add_action(&actions.next_tab);
        self.window.add_action(&actions.prev_tab);
        self.window.add_action(&actions.tab1);
        self.window.add_action(&actions.tab2);
        self.window.add_action(&actions.tab3);
        self.window.add_action(&actions.tab4);
        self.window.add_action(&actions.tab5);
        self.window.add_action(&actions.tab6);
        self.window.add_action(&actions.tab7);
        self.window.add_action(&actions.tab8);
        self.window.add_action(&actions.tab9);
        self.window.add_action(&actions.reload);
        self.window.add_action(&actions.go_home);
        self.window.add_action(&actions.go_previous);
        self.window.add_action(&actions.go_next);
        self.window.add_action(&actions.new_window);
        self.window.add_action(&actions.open_bookmarks);
        self.window.add_action(&actions.bookmark_page);
        self.window.add_action(&actions.open_history);
        self.window.add_action(&actions.clear_history);
        self.window.add_action(&actions.view_source);
        self.window.add_action(&actions.save_page);
        self.window.add_action(&actions.open_prefs);
        self.window.add_action(&actions.open_about);
        self.window.add_action(&actions.quit);
        actions
    }

    fn new_tab(&self, uri: Option<&str>) {
        let newtab = tab::Tab::init();
        self.tabs
            .borrow_mut()
            .insert(newtab.tab().widget_name().to_string(), newtab.clone());
        let cfg = CONFIG.lock().unwrap().clone();
        let uri = if cfg.general.new_page == config::NewPage::Home && uri.is_none() {
            Some(cfg.general.homepage.as_str())
        } else {
            uri
        };
        if let Some(uri) = uri {
            if let Ok(u) = Url::parse(uri) {
                let host = u.host_str().unwrap_or("Unknown host");
                newtab.label().label().set_label(host);
            }
            newtab.addr_bar().set_text(uri);
            newtab.reload_button().set_sensitive(true);
            newtab.viewer().visit(uri);
        }
        self.notebook
            .append_page(&newtab.tab(), Some(&newtab.label().handle()));
        self.notebook.set_tab_reorderable(&newtab.tab(), true);
        newtab.connect_signals();
        newtab.label().close_button().connect_clicked(
            clone!(@strong newtab as tab, @weak self.notebook as nb => move |_| {
                let _name = tab.tab().widget_name().to_string();
                nb.detach_tab(&tab.tab());
            }),
        );
        newtab.viewer().connect_page_load_started(
            clone!(@weak self.window as window, @strong newtab as tab => move |_, uri| {
                window.set_title(Some(&format!(
                    "{}-{} - [loading]",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                )));
                tab.addr_bar().set_text(&uri);
                tab.set_label("[loading]", true);
                tab.reload_button().set_sensitive(false);
            }),
        );
        newtab.viewer().connect_page_loaded(
            clone!(@strong newtab as tab, @weak self.window as window => move |_, uri| {
                tab.addr_bar().set_text(&uri);
                tab.reload_button().set_sensitive(true);
                tab.back_button().set_sensitive(tab.viewer().has_previous());
                tab.forward_button().set_sensitive(tab.viewer().has_next());
                tab.update_bookmark_editor();
                if let Ok(url) = Url::parse(uri.as_str()) {
                    let scheme = url.scheme();
                    let host = url.host_str().unwrap_or_else(|| {
                        if scheme == "file" {
                            "filesystem"
                        } else {
                            "Unknown host"
                        }
                    });
                    window.set_title(Some(&format!(
                        "{}-{} - {}",
                        env!("CARGO_PKG_NAME"),
                        env!("CARGO_PKG_VERSION"),
                        host,
                    )));
                    tab.set_label(host, false);
                }
            }),
        );
        newtab.viewer().connect_page_load_failed(
            clone!(@strong newtab as tab, @weak self.window as window => move |_, err| {
                tab.reload_button().set_sensitive(true);
                tab.back_button().set_sensitive(tab.viewer().has_previous());
                tab.forward_button().set_sensitive(tab.viewer().has_next());
                if err.contains("unsupported-scheme") {
                    if let Ok(url) = Url::parse(tab.viewer().uri().as_str()) {
                        if let Some(host) = url.host_str() {
                            tab.set_label(host, false);
                            window.set_title(Some(&format!(
                                "{}-{} -{}",
                                env!("CARGO_PKG_NAME"),
                                env!("CARGO_PKG_VERSION"),
                                host,
                            )));
                        }
                    }
                    tab.addr_bar().set_text(tab.viewer().uri().as_str());
                    return;
                }
                tab.set_label("Load failure", false);
                tab.viewer().render_gmi(&format!(
                    "# Page load failure\n\n{}",
                    match err.as_str() {
                        "RelativeUrlWithCannotBeABaseBase" => "Invalid url",
                        s if s.contains(
                            "failed to lookup address information: Name or service not known"
                        ) =>
                        {
                            "Cannot resolve dns for host"
                        }
                        s => s,
                    },
                ));
                window.set_title(Some(&format!(
                    "{}-{} - page load failed",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                )));
            }),
        );
        newtab
            .viewer()
            .connect_request_new_tab(clone!(@strong self as gui => move |_, uri| {
                gui.new_tab(Some(&uri));
            }));
        if let Some(app) = self.window.application() {
            newtab.viewer().connect_request_new_window(move |_, uri| {
                let gui = build_ui(&app);
                gui.new_tab(Some(&uri));
            });
        }
        newtab.viewer().connect_request_input(
            clone!(@strong newtab as tab, @weak self.window as window => move |_viewer, meta, url| {
                if let Ok(url) = Url::parse(&url) {
                    if let Some(host) = url.host_str() {
                        tab.set_label(host, false);
                        window.set_title(Some(&format!(
                            "{}-{} - {}",
                            env!("CARGO_PKG_NAME"),
                            env!("CARGO_PKG_VERSION"),
                            host,
                        )));
                    }
                }
                tab.addr_bar().set_text(&url);
                tab.request_input(&meta, url, true);
            }),
        );
        newtab.viewer().connect_request_input_sensitive(
            clone!(@strong newtab as tab, @weak self.window as window => move |_viewer, meta, url| {
                if let Ok(url) = Url::parse(&url) {
                    if let Some(host) = url.host_str() {
                        tab.set_label(host, false);
                        window.set_title(Some(&format!(
                            "{}-{} - {}",
                            env!("CARGO_PKG_NAME"),
                            env!("CARGO_PKG_VERSION"),
                            host,
                        )));
                    }
                }
                tab.addr_bar().set_text(&url);
                tab.request_input(&meta, url, false);
            }),
        );
        newtab.viewer().connect_request_download(
            clone!(@strong self as gui => move |viewer, mime, filename| {
                gui.download(viewer, &mime, &filename);
            }),
        );
    }

    fn download(&self, viewer: &GemView, mime: &str, filename: &str) {
        let cfg = CONFIG.lock().unwrap();
        let filename = if filename == "download" {
            if let Some(extension) = mime2ext(mime) {
                Cow::from(format!("{}.{}", filename, extension))
            } else {
                Cow::from(filename)
            }
        } else {
            Cow::from(filename)
        };
        match cfg.general.download_scheme {
            config::DownloadScheme::Ask => {
                self.dialogs.save.set_current_name(&filename);
                self.dialogs.save.connect_response(
                    clone!(@weak viewer, @strong self as gui => move |dlg,response| {
                        match response {
                            gtk::ResponseType::Accept => {
                                if let Some(file) = dlg.file() {
                                    if let Some(path) = file.path() {
                                        match fs::write(&path, &viewer.buffer_content()) {
                                            Ok(_) => gui.send_notification(&format!(
                                                "File saved: {}",
                                                path.display(),
                                            )),
                                            Err(e) => gui.send_notification(&format!(
                                                "Error: {}",
                                                e,
                                            )),
                                        }
                                    }
                                }
                                dlg.hide();
                            },
                            _ => dlg.hide(),
                        }
                    }),
                );
                self.dialogs.save.show();
                viewer.reload();
            }
            config::DownloadScheme::Auto => {
                if let Some(location) = &cfg.general.download_location {
                    let mut location = PathBuf::from(location);
                    if !location.exists() {
                        if let Err(e) = fs::create_dir_all(&location) {
                            self.send_notification(&format!("Error: {}", e,));
                            viewer.reload();
                            return;
                        }
                    }
                    location.push(&*filename);
                    match fs::write(&location, &viewer.buffer_content()) {
                        Ok(_) => {
                            self.send_notification(&format!("File saved: {}", location.display()));
                        }
                        Err(e) => self.send_notification(&format!("Error: {}", e,)),
                    }
                    viewer.reload();
                }
            }
        }
    }

    fn send_notification(&self, message: &str) {
        if let Some(application) = self.window.application() {
            let notification = Notification::new(env!("CARGO_PKG_NAME"));
            notification.set_body(Some(message));
            application.send_notification(None, &notification);
        }
    }

    fn current_page(&self) -> Option<u32> {
        self.notebook.current_page()
    }

    fn current_tab(&self) -> Option<Tab> {
        if let Some(t) = self.notebook.nth_page(self.current_page()) {
            self.tabs
                .borrow()
                .get(&t.widget_name().to_string())
                .cloned()
        } else {
            None
        }
    }

    fn nth_tab(&self, num: u32) -> Option<Tab> {
        if let Some(t) = self.notebook.nth_page(Some(num)) {
            self.tabs
                .borrow()
                .get(&t.widget_name().to_string())
                .cloned()
        } else {
            None
        }
    }

    fn next_tab(&self) {
        if let Some(current) = self.notebook.current_page() {
            let pages = self.notebook.n_pages();
            if current == pages - 1 {
                self.notebook.set_page(0);
            } else {
                self.notebook.set_page((current + 1).try_into().unwrap());
            }
        }
    }

    fn prev_tab(&self) {
        if let Some(current) = self.current_page() {
            let pages = self.notebook.n_pages();
            if current == 0 {
                self.notebook.set_page((pages - 1).try_into().unwrap());
            } else {
                self.notebook.set_page((current - 1).try_into().unwrap());
            }
        }
    }

    fn close_current_tab(&self) {
        if let Some(page) = self.current_page() {
            if let Some(tab) = self.current_tab() {
                let name = tab.tab().widget_name().to_string();
                self.tabs.borrow_mut().remove(&name);
            }
            self.notebook.remove_page(Some(page));
        }
    }

    /* fn close_tab_named(&self, name: &str) {
        match self.tabs.borrow().get(name) {
            Some(tab) => self.notebook.detach_tab(&tab.tab()),
            None => {},
        }
        self.tabs.borrow_mut().remove(name);
    } */

    fn cleanup_tabs(&self) {
        let tabs = self.tabs.borrow_mut().clone();
        for (name, tab) in tabs {
            match self.notebook.page_num(&tab.tab()) {
                Some(_) => {}
                None => {
                    let _rem = self.tabs.borrow_mut().remove(&name);
                }
            }
        }
    }

    fn reload_current_tab(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(tab) = self.current_tab() {
            tab.viewer().reload();
            Ok(())
        } else {
            Err(String::from("Error getting tab").into())
        }
    }

    fn go_home(&self) -> Result<(), Box<dyn std::error::Error>> {
        let home = CONFIG.lock().unwrap().clone().general.homepage;
        if let Some(tab) = self.current_tab() {
            tab.viewer().visit(&home);
            Ok(())
        } else {
            Err(String::from("Error getting tab").into())
        }
    }

    fn go_previous(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(tab) = self.current_tab() {
            tab.viewer().go_previous();
            Ok(())
        } else {
            Err(String::from("Error getting tab").into())
        }
    }

    fn go_next(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(tab) = self.current_tab() {
            tab.viewer().go_next();
            Ok(())
        } else {
            Err(String::from("Error getting tab").into())
        }
    }

    fn switch_tab(&self, page: u32) {
        if let Some(tab) = self.nth_tab(page) {
            let uri = tab.viewer().uri();
            if let Ok(url) = Url::parse(uri.as_str()) {
                self.window.set_title(Some(&format!(
                    "{}-{} - {}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION"),
                    url.host_str().unwrap_or("Unknown host"),
                )));
            }
        }
    }

    fn set_show_tabs(&self, show: &config::ShowTabs) {
        self.notebook.set_show_tabs(match show {
            config::ShowTabs::Always => true,
            config::ShowTabs::Never => false,
            config::ShowTabs::Multiple => self.notebook.n_pages() > 1,
        });
    }

    fn set_tab_position(&self, pos: &config::TabPosition) {
        self.notebook.set_tab_pos(pos.to_gtk());
    }

    fn set_general(&self, gen: &config::General) {
        self.set_show_tabs(&gen.show_tabs);
        self.set_tab_position(&gen.tab_position);
    }

    fn set_css(&self, colors: &config::Colors) {
        let provider = CssProvider::new();
        let context = self.window.style_context();
        let css = include_str!("gemview.css")
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
        provider.load_from_data(css.as_bytes());
        StyleContext::add_provider_for_display(
            &Display::default().expect("Cannot connect to display"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn open_bookmarks(&self) {
        if let Some(tab) = self.current_tab() {
            tab.open_bookmarks();
            self.window.set_title(Some(&format!(
                "{}-{} - bookmarks",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
            )));
        }
    }

    fn save_page(&self) {
        if let Some(tab) = self.current_tab() {
            let viewer = tab.viewer();
            let mut filename = if let Some(s) = viewer.uri().split('/').last() {
                match s {
                    "" => "unknown",
                    _ => s,
                }
            } else {
                "unknown"
            }
            .to_string();
            if !filename.contains('.') {
                let mime = viewer.buffer_mime();
                let ext = if let Some(e) = mime2ext(&mime) {
                    Some(e)
                } else if mime == "text/gemini" {
                    Some("gmi")
                } else {
                    None
                };
                if let Some(ext) = ext {
                    filename.push('.');
                    filename.push_str(ext);
                }
            }
            self.dialogs.save.set_current_name(&filename);
            self.dialogs.save.connect_response(
                clone!(@weak viewer, @strong self as gui => move |dlg,response| {
                    match response {
                        gtk::ResponseType::Accept => {
                            if let Some(file) = dlg.file() {
                                if let Some(path) = file.path() {
                                    match fs::write(&path, &viewer.buffer_content()) {
                                        Ok(_) => gui.send_notification(&format!(
                                            "File saved: {}",
                                            path.display(),
                                        )),
                                        Err(e) => gui.send_notification(&format!(
                                            "Error: {}",
                                            e,
                                        )),
                                    }
                                }
                            }
                            dlg.hide();
                        },
                        _ => dlg.hide(),
                    }
                }),
            );
            self.dialogs.save.show();
            viewer.reload();
        }
    }
}

pub fn run() {
    let application = Rc::new(gtk::Application::new(
        Some("org.hitchhiker-linux.eva"),
        gtk::gio::ApplicationFlags::HANDLES_OPEN,
    ));

    application.add_main_option(
        "private",
        Char::from(b'p'),
        OptionFlags::NONE,
        OptionArg::None,
        "Do not save history",
        None,
    );

    application.add_main_option(
        "version",
        Char::from(b'v'),
        OptionFlags::NONE,
        OptionArg::None,
        "Display program version",
        None,
    );

    application.connect_handle_local_options(move |_, dict| {
        if dict.contains("version") {
            println!("{}", env!("CARGO_PKG_VERSION"));
            return 1;
        }
        -1
    });

    match application.register(Some(&Cancellable::new())) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    };

    application.connect_open(move |app, addr, _| {
        let gui = build_ui(app);
        for uri in addr {
            gui.new_tab(Some(&uri.uri()));
        }
    });
    application.connect_activate(|app| {
        let gui = build_ui(app);
        gui.new_tab(None);
    });
    application.run();
}

fn build_ui(app: &Application) -> Rc<Gui> {
    let gui = Rc::new(Gui::default());
    gui.add_actions(app).connect(&gui, app);
    let config = CONFIG.lock().unwrap().clone();
    gui.set_css(&config.colors);
    gui.window.set_application(Some(app));
    gui.notebook
        .connect_page_removed(clone!(@weak gui, @strong config => move |nb,_page,_| {
            gui.cleanup_tabs();
            let multi = config.general.show_tabs == config::ShowTabs::Multiple;
            match nb.n_pages() {
                0 => gui.window.close(),
                1 => if multi { nb.set_show_tabs(false); },
                _ => if multi { nb.set_show_tabs(true); },
            }
        }));
    gui.notebook
        .connect_page_added(clone!(@weak gui, @strong config => move |nb,_page,_| {
            if nb.n_pages() > 1 && config.general.show_tabs == config::ShowTabs::Multiple {
                nb.set_show_tabs(true);
            }
        }));
    gui.notebook
        .connect_switch_page(clone!(@weak gui => move |_,_,page| {
            gui.switch_tab(page);
        }));
    gui.dialogs
        .preferences
        .window()
        .connect_response(clone!(@weak gui => move |dlg,res| {
            if res == ResponseType::Accept {
                if let Some(cfg) = gui.dialogs.preferences.config() {
                    *CONFIG.lock().unwrap() = cfg.clone();
                    cfg.save_to_file(&config::get_config_file());
                    gui.set_general(&cfg.general);
                    gui.set_css(&cfg.colors);
                    for (_,tab) in gui.tabs.borrow().clone() {
                        tab.set_fonts();
                    }
                } else {
                    gui.dialogs.preferences.load_config();
                }
            }
            dlg.hide();
        }));
    gui.set_general(&config.general);

    gui.window.show();
    gui
}
