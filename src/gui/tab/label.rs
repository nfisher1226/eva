use gtk::prelude::*;

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

    pub fn set(&self, label: &str, spin: bool) {
        self.label.set_label(label);
        if spin {
            self.spinner.show();
            self.spinner.start();
        } else {
            self.spinner.stop();
            self.spinner.hide();
        }
    }
}
