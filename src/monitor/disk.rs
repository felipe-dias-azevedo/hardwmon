use sysinfo::{DiskExt, DiskType, ProcessExt, System, SystemExt};

pub struct DisksData {
    pub disks: Vec<DiskData>,
    pub read_total: u64,
    pub write_total: u64
}

pub struct DiskData {
    pub name: String,
    pub disk_type: Option<String>,
    pub file_system: Option<String>,
    pub mount_point: String,
    pub removable: bool,
    pub space_available: f64,
    pub space_total: f64,
    pub space_used: f64
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
                mount_point: String::from(mount_point.into()),
                removable: is_removable
            }
        }).collect::<Vec<DiskData>>();

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

        let Some((read, write)) = disk_usage;

        DisksData {
            read_total: read,
            write_total: write,
            disks: disks_data
        }
    }
}