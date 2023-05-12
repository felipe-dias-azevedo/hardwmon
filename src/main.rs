mod monitor;

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
//use crate::monitor::get_data;

fn main() {

    gtk::init().expect("GTK failed");

    let app = gtk::Application::new(
        Some("com.felipe.hardwmon"),
        ApplicationFlags::HANDLES_OPEN,
    );

    let builder = gtk::Builder::from_file("src/assets/windows/hardwmon-window.glade");
    let window: gtk::Window = builder.object("appwindow").expect("Couldn't set window");

    window.show_all();

    app.connect_activate(clone!(@strong window => move |app| {
        if let Some(existing_window) = app.active_window() {
            existing_window.present();
        } else {
            window.set_application(Some(app));
            app.add_window(&window);
        }
    }));

    window.connect_delete_event(|_, _| {
        gtk::main_quit();

        Inhibit(false)
    });

    let headerpaned: gtk::Paned = builder
        .object("headerpaned")
        .expect("Couldn't get headerpaned");

    let contentpaned: gtk::Paned = builder
        .object("contentpaned")
        .expect("Couldn't get contentpaned");

    let subheader: gtk::HeaderBar = builder
        .object("subheader")
        .expect("Couldn't get subheader");

    let subcontent: gtk::Box = builder
        .object("subcontent")
        .expect("Couldn't get subcontent");

    let headerpanelbutton: gtk::Button = builder
        .object("headerpanelbutton")
        .expect("Couldn't get headerpanelbutton");

    let maintreeview: gtk::TreeView = builder
        .object("maintreeview")
        .expect("Couldn't get maintreeview");

    headerpanelbutton.connect_clicked(clone!(@strong subheader, @strong subcontent => move |_| {
        subheader.set_visible(!subheader.is_visible());
        subcontent.set_visible(!subcontent.is_visible());
    }));

    // let data = get_data();
    //
    //
    // maintreeview.set_model(Some(&data));
    //
    // fn expander_cell_data_func(
    //     tree_view: &gtk::TreeView,
    //     expander: &gtk::CellRendererToggle,
    //     path: &gtk::TreePath,
    // ) {
    //     let tree_model = tree_view.model().unwrap();
    //     let iter = tree_model.iter(path).unwrap();
    //
    //     // Get the value of the "has_children" column for the current row
    //     let has_children: bool = tree_model
    //         .get_value(&iter, 0i32)
    //         .get()
    //         .unwrap();
    //
    //     // Set the "active" property of the expander based on the "has_children" value
    //     expander.set_active(has_children);
    // }
    // let expander_cell_data_func_box= Some(Box::new(expander_cell_data_func));
    //
    // // Define the columns
    // let column = gtk::TreeViewColumn::new();
    // let renderer = gtk::CellRendererText::new();
    // column.pack_start(&renderer, true);
    // column.add_attribute(&renderer, "text", 0);
    // column.set_title("Name");
    // maintreeview.append_column(&column);
    //
    // let column = gtk::TreeViewColumn::new();
    // let renderer = gtk::CellRendererText::new();
    // column.pack_start(&renderer, true);
    // column.add_attribute(&renderer, "text", 1);
    // column.set_title("Value");
    // maintreeview.append_column(&column);
    //
    // // Set the expander cell data function
    // let renderer = gtk::CellRendererText::new();
    // let column = gtk::TreeViewColumn::new();
    // column.pack_start(&renderer, true);
    // column.set_cell_data_func(&renderer, expander_cell_data_func_box);
    // maintreeview.append_column(&column);
    //
    // // Create a scrolled window and add the tree view to it
    // let scrolled_window = gtk::ScrolledWindow::new(None, None);
    // scrolled_window.set_vexpand(true);
    // scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    // scrolled_window.add(&maintreeview);



    let nvidia = Nvml::init();
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut cpupc = CpuPercentCollector::new().unwrap();

    {
        println!("------ SYSTEM -----");
        let system = sys.long_os_version().unwrap_or_default();
        let boot_time = sys.boot_time();
        let distro = sys.distribution_id();
        let hostname = sys.host_name().unwrap_or_default();

        println!("Name: {}", system);
        println!("Hostname: {}", hostname);
        println!("Boot Time: {}", boot_time);
        println!("Distro: {}", distro);
    }

    {
        println!("------ RAM -----");
        let total = sys.total_memory() as f64;
        let available = sys.free_memory() as f64;
        let used = sys.used_memory() as f64;

        println!(
            "Usage: {:.2} GB / {:.2} GB / {:.2} %",
            used / 1_000_000_000.0,
            total / 1_000_000_000.0,
            (used / total) * 100.0
        );
        println!(
            "Available: {:.2} GB / {:.2} GB / {:.2} %",
            available / 1_000_000_000.0,
            total / 1_000_000_000.0,
            ((total - available) / total) * 100.0
        )
    }

    {
        println!("------ SWAP -----");
        let total = sys.total_swap() as f64;
        let available = sys.free_swap() as f64;
        let used = sys.used_swap() as f64;

        println!(
            "Usage: {:.2} GB / {:.2} GB / {:.2} %",
            used / 1_000_000_000.0,
            total / 1_000_000_000.0,
            (used / total) * 100.0
        );
        println!("Available: {:.2} GB", available / 1_000_000_000.0)
    }

    {
        println!("------ Network -----");
        let networks = sys.networks();
        for (interface, data) in networks {
            println!("Interface: {}", interface);
            println!("Download: {} Bytes", data.received());
            println!("Upload: {} Bytes", data.transmitted());
            println!();
        }
    }

    {
        println!("------ Disks -----");
        let disks = sys.disks();
        for d in disks {
            let name = d.name();
            let disk_type = d.type_();
            let file_system = d.file_system();
            let space = d.total_space() as f64;
            let available_space = d.available_space() as f64;
            let mount_point = d.mount_point();
            let is_removable = d.is_removable();

            println!("Name: {:?}", name);
            println!("Type: {:?}", disk_type);
            if let Ok(file_system) = String::from_utf8(Vec::from(file_system)) {
                println!("File System: {}", file_system);
            }
            println!(
                "Space: {:.2} GB / {:.2} GB / {:.2} %",
                (space - available_space) / 1_000_000_000.0,
                space / 1_000_000_000.0,
                ((space - available_space) / space) * 100.0
            );
            println!("Mounted: {:?}", mount_point);
            println!("Removable: {}", is_removable);
            println!();
        }

        let disk_usage = sys
            .processes()
            .into_iter()
            .filter_map(|(_pid, process)| {
                let disk_usage = process.disk_usage();
                let read = disk_usage.read_bytes;
                let write = disk_usage.written_bytes;

                if read == 0 || write == 0 {
                    return None;
                }

                return Some((read, write));
            })
            .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1));

        if let Some((read, write)) = disk_usage {
            println!();
            println!("Read: {} Bytes", read);
            println!("Write: {} Bytes", write);
        }
    }

    {
        println!("------ Sensors -----");
        let temperatures = temperatures();
        for t in temperatures {
            if let Some(temp) = t.ok() {
                let unit = temp.unit();
                let celsius = temp.current().celsius().round();

                println!("Unit: {}", unit);

                if let Some(max) = temp.high() {
                    let max = max.celsius().round();
                    println!("Temperature: {:.0} ºC (MAX: {:.0} ºC)", celsius, max);
                } else {
                    println!("Temperature: {:.0} ºC", celsius);
                }

                if let Some(label) = temp.label() {
                    println!("Label: {}", label);
                }
                println!();
            }
        }
    }

    {
        println!("------ CPU -----");
        let cpus = sys.cpus();
        let brand = sys.global_cpu_info().brand();
        // let freq = sys.global_cpu_info().frequency();
        // let usage = sys.global_cpu_info().cpu_usage();

        println!("Name: {}", brand);

        for i in 0..cpus.len() {
            let cpu = &cpus[i];

            let freq = cpu.frequency();
            let usage = cpu.cpu_usage();
    
            println!("[CPU {}] Frequency: {} MHz", i, freq);
            println!("[CPU {}] Usage: {:.0}%", i, usage);
            println!();
        }
    }

    if let Some(nvidia) = nvidia.ok() {
        let device = nvidia.device_by_index(0);

        if let Some(device) = device.ok() {
            println!("------ GPU -----");
            let name = device.name().unwrap();
            let fan_speed = device.fan_speed(0).unwrap();
            let memory_info = device.memory_info().unwrap();
            let power_usage = device.power_usage().unwrap();
            let temperature = device.temperature(TemperatureSensor::Gpu).unwrap();
            let utilization = device.utilization_rates().unwrap();

            println!("Name: {}", name);
            println!("Fan Speed: {}%", fan_speed);
            println!(
                "Memory: {} MB / {} MB / {}%",
                memory_info.used / 1000000,
                memory_info.total / 1000000,
                round(
                    memory_info.used as f64 / memory_info.total as f64 * 100.0,
                    1
                )
            );
            println!("Power: {} W", power_usage / 1000);
            println!("Temperature: {} C", temperature);
            println!("Usage: {}% (GPU)", utilization.gpu);
            println!("Usage: {}% (MEM)", utilization.memory);
        }
    }

    gtk::main();
}
