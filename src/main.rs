mod monitor;
mod views;


use std::thread;
use std::time::Duration;
use nvml_wrapper::{Nvml};
use sysinfo::{System, SystemExt};
use crate::monitor::{MonitorData, MonitorRow};

use crate::views::window::Window;
use crate::views::app::Application;

fn main() {
    gtk::init().expect("GTK failed");

    let app = Application::new();

    let (sender, receiver) = gtk::glib::MainContext::channel::<MonitorData>(gtk::glib::PRIORITY_DEFAULT);

    let builder = gtk::Builder::from_file("src/assets/windows/hardwmon-window.glade");
    let window = Window::new(&builder);

    app.on_activate(&window);
    window.on_close();
    window.on_about_clicked(&builder);
    window.on_sidebar_toggle_clicked();

    let nvidia = Nvml::init();
    let mut sys = System::new_all();

    sys.refresh_all();
    let data = monitor::get_hardware_data(&sys, &nvidia);
    let tree_model = monitor::get_tree_model(data);
    window.set_treeview(tree_model);

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            sys.refresh_all();

            let data = monitor::get_hardware_data(&sys, &nvidia);

            sender.send(MonitorData { data }).unwrap();

        }
    });

    receiver.attach(None, move |monitor| {
        // let tree_model = monitor::get_tree_model(monitor.data);

        window.update_treeview(MonitorRow::on_single_list(monitor.data));

        gtk::glib::Continue(true)
    });

    app.start();
    //gtk::main();
}
