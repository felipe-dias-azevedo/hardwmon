mod monitor;
mod views;


use nvml_wrapper::{Nvml};
use sysinfo::{System, SystemExt};

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

    let data = monitor::get_hardware_data(&sys, &nvidia);
    let tree_model = monitor::get_tree_model(data);

    window.set_treeview(tree_model);

    app.start();
    //gtk::main();
}
