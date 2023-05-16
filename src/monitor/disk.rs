use sysinfo::{DiskExt, ProcessExt, System, SystemExt};

pub fn get_disk_data(sys: &System) {
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
}