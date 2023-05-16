use crate::views::window::Window;
use gtk::gio::{ApplicationFlags, Cancellable};
use gtk::prelude::*;
use gtk::glib::clone;


static APP_ID: Option<&str> = Some("com.felipe.hardwmon");

pub struct Application {
    app: gtk::Application
}

impl Application {
    pub fn new() -> Self {
        let application = Application {
            app: gtk::Application::new(
                APP_ID,
                ApplicationFlags::HANDLES_OPEN,
            )
        };

        application.app.register(Cancellable::NONE).expect("Register failed");
        //application.app.activate();

        application
    }

    pub fn on_activate(&self, window: &Window) {
        self.app
            .connect_activate(clone!(@strong window.window as window => move |app| {
                if let Some(existing_window) = app.active_window() {
                    existing_window.present();
                } else {
                    window.set_application(Some(app));
                    app.add_window(&window);
                }
            }));
    }

    pub fn start(&self) {
        self.app.run();
    }
}