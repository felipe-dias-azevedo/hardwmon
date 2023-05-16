use sysinfo::{System, SystemExt};

pub struct SystemData {
    pub system: String,
    pub boot_time: u64,
    pub distro: String,
    pub hostname: String
}

pub fn get_system_data(sys: &System) {
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
}

