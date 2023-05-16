mod monitor;
mod views;

use gtk::{
    gio::ApplicationFlags,
    prelude::*,
};
use gtk::glib::clone;
use nvml_wrapper::{enum_wrappers::device::TemperatureSensor, Nvml};
use psutil::cpu::CpuPercentCollector;
use psutil::sensors::temperatures;
use psutil::{Degrees, Temperature};
use round::round;
use sysinfo::{CpuExt, DiskExt, NetworkExt, ProcessExt, System, SystemExt};

use crate::views::window::Window;
use crate::views::app::Application;

fn main() {

    gtk::init().expect("GTK failed");

    let app = Application::new();

    let builder = gtk::Builder::from_file("src/assets/windows/hardwmon-window.glade");
    let window = Window::new(&builder);

    app.on_activate(&window);
    window.on_close();
    window.on_about_clicked(&builder);
    window.on_sidebar_toggle_clicked();
    
    let nvidia = Nvml::init();
    let mut sys = System::new_all();
    sys.refresh_all();

    let tree_model = monitor::get_tree_model();

    window.set_treeview(tree_model);

    gtk::main();
}
