use nvml_wrapper::{Nvml, enum_wrappers::device::TemperatureSensor};
use round::round;
use sysinfo::{CpuExt, DiskExt, NetworkExt, ProcessExt, System, SystemExt};
use psutil::cpu::CpuPercentCollector;
use psutil::sensors::temperatures;
use psutil::{Degrees, Temperature};

fn main() {
    let nvidia = Nvml::init();
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut cpupc = CpuPercentCollector::new().unwrap();

    {
        println!("------ Network -----");
        let networks = sys.networks();
        for (interface, data) in networks {
            println!("Interface: {}", interface);
            println!("Download: {} Bytes", data.received());
            println!("Upload: {} Bytes",data.transmitted());
        }
    }

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
            println!("Space: {:.2} GB / {:.2} GB / {:.2} %", (space - available_space) / 1_000_000_000.0, space / 1_000_000_000.0, ((space - available_space) / space) * 100.0);
            println!("Mounted: {:?}", mount_point);
            println!("Removable: {}", is_removable);
        }

        let disk_usage = sys.processes().into_iter().filter_map(|(_pid,  process)| {
            let disk_usage = process.disk_usage();
            let read = disk_usage.read_bytes;
            let write = disk_usage.written_bytes;

            if read == 0 || write == 0 {
                return None;
            }

            return Some((read, write));
        }).reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1));

        if let Some((read, write)) = disk_usage {
            println!();
            println!("Read: {} Bytes", read);
            println!("Write: {} Bytes", write);
        }
    }

    {
        println!("------ Sensors -----");
        let temperatures = temperatures();
        for t in temperatures {
            if let Some(temp) = t.ok() {
                let unit = temp.unit();
                let celsius = temp.current().celsius().round();

                println!("Unit: {}", unit);

                if let Some(max) = temp.high() {
                    let max = max.celsius().round();
                    println!("Temperature: {:.0} ºC (MAX: {:.0} ºC)", celsius, max);
                } else {
                    println!("Temperature: {:.0} ºC", celsius);
                }

                if let Some(label) = temp.label() {
                    println!("Label: {}", label);
                }
            }
        }
    }

    {
        println!("------ CPU -----");
        let brand = sys.global_cpu_info().brand();
        let freq = sys.global_cpu_info().frequency();
        let usage = sys.global_cpu_info().cpu_usage();

        println!("Name: {}", brand);
        println!("Frequency: {} MHz", freq);
        println!("Usage: {:.0}%", usage);
    }


    if let Some(nvidia) = nvidia.ok() {
        let device = nvidia.device_by_index(0);

        if let Some(device) = device.ok() {
            println!("------ GPU -----");
            let name = device.name().unwrap();
            let fan_speed = device.fan_speed(0).unwrap();
            let memory_info = device.memory_info().unwrap();
            let power_usage = device.power_usage().unwrap();
            let temperature = device.temperature(TemperatureSensor::Gpu).unwrap();
            let utilization = device.utilization_rates().unwrap();

            println!("Name: {}", name);
            println!("Fan Speed: {}%", fan_speed);
            println!("Memory: {} MB / {} MB / {}%", memory_info.used / 1000000, memory_info.total / 1000000, round(memory_info.used as f64 / memory_info.total as f64 * 100.0, 1));
            println!("Power: {} W", power_usage / 1000);
            println!("Temperature: {} C", temperature);
            println!("Usage: {}% (GPU)", utilization.gpu);
            println!("Usage: {}% (MEM)", utilization.memory);
        }
    }
}
