use sysinfo::{DiskExt, DiskType, ProcessExt, System, SystemExt};
use crate::monitor::MonitorRow;
use crate::views::get_bytevalue_from;

pub struct DisksData {
    pub disks: Vec<DiskData>,
    pub read_total: Option<u64>,
    pub write_total: Option<u64>,
}

pub struct DiskData {
    pub name: String,
    pub disk_type: Option<String>,
    pub file_system: Option<String>,
    pub mount_point: String,
    pub removable: bool,
    pub space_available: f64,
    pub space_total: f64,
    pub space_used: f64,
}

impl DisksData {
    pub fn new(sys: &System) -> DisksData {
        let disks = sys.disks();

        let disks_data = disks.into_iter().map(|d| {
            let name = d.name();
            let disk_type = d.type_();
            let file_system = d.file_system();
            let space = d.total_space() as f64;
            let available_space = d.available_space() as f64;
            let mount_point = d.mount_point();
            let is_removable = d.is_removable();

            DiskData {
                name: name.to_os_string().into_string().unwrap_or_default(),
                disk_type: match disk_type {
                    DiskType::SSD => Some(String::from("SSD")),
                    DiskType::HDD => Some(String::from("HD")),
                    _ => None
                },
                file_system: String::from_utf8(Vec::from(file_system)).ok(),
                space_total: space,
                space_available: available_space,
                space_used: space - available_space,
                mount_point: String::from(mount_point.to_str().unwrap_or_default()),
                removable: is_removable,
            }
        }).collect::<Vec<DiskData>>();

        let disk_usage = sys
            .processes()
            .into_iter()
            .filter_map(|(_pid, process)| {
                let disk_usage = process.disk_usage();
                let read = disk_usage.read_bytes;
                let write = disk_usage.written_bytes;

                if read == 0 && write == 0 {
                    return None;
                }

                return Some((read, write));
            })
            .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1));

        let (read, write) = match disk_usage {
            Some(x) => (Some(x.0), Some(x.1)),
            _ => (None, None)
        };

        DisksData {
            read_total: read,
            write_total: write,
            disks: disks_data,
        }
    }

    pub fn format(&self) -> MonitorRow {
        MonitorRow {
            title: String::from("DISK"),
            value: None,
            child: vec![
                MonitorRow {
                    title: String::from("Read Total"),
                    value: match self.read_total {
                        Some(x) => Some(get_bytevalue_from(x)),
                        _ => Some(get_bytevalue_from(0))
                    },
                    child: vec![],
                },
                MonitorRow {
                    title: String::from("Write Total"),
                    value: match self.write_total {
                        Some(x) => Some(get_bytevalue_from(x)),
                        _ => Some(get_bytevalue_from(0))
                    },
                    child: vec![],
                },
                MonitorRow {
                    title: String::from("Disks"),
                    value: None,
                    child: self.disks.iter().map(|d| {
                        MonitorRow {
                            title: format!("{}", d.name),
                            value: None,
                            child: vec![
                                MonitorRow {
                                    title: format!("{} Disk Type", d.name),
                                    value: d.disk_type.to_owned(),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} File System", d.name),
                                    value: d.file_system.to_owned(),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} Mount point", d.name),
                                    value: Some(d.mount_point.to_owned()),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} Removable", d.name),
                                    value: Some(d.removable.to_string()),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} Space Used", d.name),
                                    value: Some(format!("{:.2} GB", d.space_used / 1_000_000_000f64)),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} Space Total", d.name),
                                    value: Some(format!("{:.2} GB", d.space_total / 1_000_000_000f64)),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} Space Available", d.name),
                                    value: Some(format!("{:.2} GB", d.space_available / 1_000_000_000f64)),
                                    child: vec![],
                                },
                                MonitorRow {
                                    title: format!("{} Space Usage", d.name),
                                    value: Some(format!("{:.2}%", (d.space_used / d.space_total) * 100f64)),
                                    child: vec![],
                                },
                            ],
                        }
                    }).collect(),
                },
            ],
        }
    }
}