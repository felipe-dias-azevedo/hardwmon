use sysinfo::{NetworkExt, System, SystemExt};

pub struct NetworkData {
    pub interface: String,
    pub download_total: u64,
    pub upload_total: u64
}

impl NetworkData {
    pub fn new(sys: &System) -> Vec<NetworkData> {
        let networks = sys.networks();

        networks.into_iter().map(|(interface, data)| {
            NetworkData {
                interface: String::from(interface),
                download_total: data.received(),
                upload_total: data.transmitted()
            }
        }).collect()
    }
}