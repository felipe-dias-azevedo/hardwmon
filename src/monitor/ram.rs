use sysinfo::{System, SystemExt};
use crate::monitor::MonitorRow;

pub struct RamData {
    pub ram_available: f64,
    pub ram_total: f64,
    pub ram_used: f64,
    pub swap_available: f64,
    pub swap_total: f64,
    pub swap_used: f64,
}

impl RamData {
    pub fn new(sys: &System) -> RamData {
        let ram_total = sys.total_memory() as f64;
        let ram_available = sys.free_memory() as f64;
        let ram_used = sys.used_memory() as f64;

        let swap_total = sys.total_swap() as f64;
        let swap_available = sys.free_swap() as f64;
        let swap_used = sys.used_swap() as f64;

        RamData {
            ram_available,
            ram_total,
            ram_used,
            swap_available,
            swap_total,
            swap_used
        }
    }

    pub fn format(&self) -> MonitorRow {
        MonitorRow {
            title: String::from("MEMORY"),
            value: None,
            child: vec![
                MonitorRow {
                    title: String::from("RAM"),
                    value: None,
                    child: vec![
                        MonitorRow {
                            title: String::from("RAM Used"),
                            value: Some(format!("{:.2} GB", self.ram_used / 1_000_000_000f64)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("RAM Total"),
                            value: Some(format!("{:.2} GB", self.ram_total / 1_000_000_000f64)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("RAM Available"),
                            value: Some(format!("{:.2} GB", self.ram_available / 1_000_000_000f64)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("RAM Usage"),
                            value: Some(format!("{:.2}%", (self.ram_used / self.ram_total) * 100f64)),
                            child: vec![]
                        }
                    ]
                },
                MonitorRow {
                    title: String::from("Swap"),
                    value: None,
                    child: vec![
                        MonitorRow {
                            title: String::from("SWAP Used"),
                            value: Some(format!("{:.2} GB", self.swap_used / 1_000_000_000f64)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("SWAP Total"),
                            value: Some(format!("{:.2} GB", self.swap_total / 1_000_000_000f64)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("SWAP Available"),
                            value: Some(format!("{:.2} GB", self.swap_available / 1_000_000_000f64)),
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("SWAP Usage"),
                            value: Some(format!("{:.2}%", (self.swap_used / self.swap_total) * 100f64)),
                            child: vec![]
                        }
                    ]
                },
            ]
        }
    }
}