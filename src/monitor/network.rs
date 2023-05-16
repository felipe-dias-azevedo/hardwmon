use sysinfo::{NetworkExt, System, SystemExt};

pub fn get_network_data(sys: &System) {
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
}