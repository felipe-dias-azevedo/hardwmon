use sysinfo::{NetworkExt, System, SystemExt};
use crate::monitor::MonitorRow;

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

    pub fn format(networks: Vec<NetworkData>) -> MonitorRow {
        MonitorRow {
            title: String::from("NETWORK"),
            value: None,
            child: networks.iter().map(|network| {

                MonitorRow {
                    title: network.interface.to_owned(),
                    value: None,
                    child: vec![
                        MonitorRow {
                            title: String::from("Download Total"),
                            value: Some(format!("{} Bytes", network.download_total)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("Upload Total"),
                            value: Some(format!("{} Bytes", network.upload_total)),
                            child: vec![]
                        }
                    ]
                }
            }).collect()
        }
    }
}