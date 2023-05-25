use sysinfo::{CpuExt, System, SystemExt};
use crate::monitor::MonitorRow;

pub struct CpuData {
    pub brand: String,
    pub usage_total: f32,
    pub frequency_total: u64,
    pub usage: Vec<f32>,
    pub frequency: Vec<u64>,
}

impl CpuData {
    pub fn new(sys: &System) -> CpuData {
        let cpus = sys.cpus();
        let brand = sys.global_cpu_info().brand();
        let frequency_total = sys.global_cpu_info().frequency();
        let usage_total = sys.global_cpu_info().cpu_usage();

        let (usage, frequency): (Vec<f32>, Vec<u64>) = cpus.into_iter()
            .map(|cpu| (cpu.cpu_usage(), cpu.frequency()))
            .unzip();

        CpuData {
            brand: String::from(brand),
            frequency_total,
            usage_total,
            usage,
            frequency,
        }
    }

    pub fn format(&self) -> MonitorRow {
        MonitorRow {
            title: String::from("CPU"),
            value: None,
            child: vec![
                MonitorRow {
                    title: String::from("Brand"),
                    value: Some(self.brand.to_owned()),
                    child: vec![],
                },
                MonitorRow {
                    title: String::from("Frequency Total"),
                    value: Some(format!("{} MHz", self.frequency_total)),
                    child: vec![],
                },
                MonitorRow {
                    title: String::from("Usage Total"),
                    value: Some(format!("{:.0}%", self.usage_total)),
                    child: vec![],
                },
                MonitorRow {
                    title: String::from("Frequency per Core"),
                    value: None,
                    child: self.frequency.iter().enumerate().map(|(i, f)| {
                        MonitorRow {
                            title: format!("Frequency Core {}", i),
                            value: Some(format!("{} MHz", f)),
                            child: vec![],
                        }
                    }).collect(),
                },
                MonitorRow {
                    title: String::from("Usage per Core"),
                    value: None,
                    child: self.usage.iter().enumerate().map(|(i, f)| {
                        MonitorRow {
                            title: format!("Usage Core {}", i),
                            value: Some(format!("{:.0}%", f)),
                            child: vec![],
                        }
                    }).collect(),
                },
            ],
        }
    }
}