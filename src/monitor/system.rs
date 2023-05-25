use sysinfo::{System, SystemExt};

pub struct SystemData {
    pub system: String,
    pub boot_time: u64,
    pub distro: String,
    pub hostname: String,
}

impl SystemData {
    pub fn new(sys: &System) -> SystemData {
        let system = sys.long_os_version().unwrap_or_default();
        let boot_time = sys.boot_time();
        let distro = sys.distribution_id();
        let hostname = sys.host_name().unwrap_or_default();

        SystemData {
            system,
            boot_time,
            distro,
            hostname,
        }
    }
}