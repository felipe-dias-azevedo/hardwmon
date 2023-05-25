use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Device, Nvml};
use crate::monitor::MonitorRow;

pub struct GpuData {
    name: String,
    fans_speed: Vec<u32>,
    memory_used: Option<f64>,
    memory_available: Option<f64>,
    memory_total: Option<f64>,
    power_usage: Option<u32>,
    temperature: Option<u32>,
    usage_gpu: Option<u32>,
    usage_memory: Option<u32>,
}

impl GpuData {
    pub fn new(nvidia: &Result<Nvml, NvmlError>) -> Vec<GpuData> {
        if let Some(nvidia) = nvidia.as_ref().ok() {
            return Self::get_nvidia_gpus(nvidia);
        }

        Vec::new()
    }

    fn get_nvidia_gpus(nvidia: &Nvml) -> Vec<GpuData> {
        let gpus_count = nvidia.device_count();

        let mut gpus = Vec::new();

        if let Some(count) = gpus_count.ok() {
            for i in 0..count {
                let device = nvidia.device_by_index(i);

                if let Some(device) = device.ok() {
                    let gpu = Self::format_nvidia_gpu_data(device);

                    gpus.push(gpu);
                }
            }
        }

        gpus
    }

    fn format_nvidia_gpu_data(device: Device) -> GpuData {
        let name = device.name().unwrap_or_default();

        let fans_speed = match device.num_fans().ok() {
            Some(fans_count) => (0..fans_count)
                .filter_map(|i| device.fan_speed(i).ok())
                .collect::<Vec<u32>>(),
            _ => Vec::new()
        };

        let (memory_used, memory_total, memory_available) = match device.memory_info().ok() {
            Some(mem) => (Some(mem.used as f64), Some(mem.total as f64), Some(mem.free as f64)),
            _ => (None, None, None),
        };
        let power_usage = device.power_usage().ok();
        let temperature = device.temperature(TemperatureSensor::Gpu).ok();
        let (usage_gpu, usage_memory) = match device.utilization_rates().ok() {
            Some(util) => (Some(util.gpu), Some(util.memory)),
            _ => (None, None),
        };

        GpuData {
            name,
            fans_speed,
            memory_used,
            memory_total,
            memory_available,
            power_usage,
            temperature,
            usage_gpu,
            usage_memory,
        }
    }

    pub fn format(gpus: Vec<GpuData>) -> MonitorRow {
        MonitorRow {
            title: String::from("GPU"),
            value: None,
            child: gpus.iter().map(|gpu| {

                MonitorRow {
                    title: gpu.name.to_owned(),
                    value: None,
                    child: vec![
                        MonitorRow {
                            title: String::from("Power Usage"),
                            value: match gpu.power_usage {
                                Some(x) => Some(format!("{} W", x / 1_000)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("Temperature"),
                            value: match gpu.temperature {
                                Some(x) => Some(format!("{} ºC", x)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("VRAM Used"),
                            value: match gpu.memory_used {
                                Some(x) => Some(format!("{:.2} GB", x / 1_000_000_000f64)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("VRAM Total"),
                            value: match gpu.memory_total {
                                Some(x) => Some(format!("{:.2} GB", x / 1_000_000_000f64)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("VRAM Available"),
                            value: match gpu.memory_available {
                                Some(x) => Some(format!("{:.2} GB", x / 1_000_000_000f64)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("VRAM Usage"),
                            value: match (gpu.memory_used, gpu.memory_total) {
                                (Some(used), Some(total)) => Some(format!("{:.2}%", (used / total) * 100f64)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("GPU Usage"),
                            value: match gpu.usage_gpu {
                                Some(x) => Some(format!("{}%", x)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("GPU Memory Usage"),
                            value: match gpu.usage_memory {
                                Some(x) => Some(format!("{}%", x)),
                                _ => Some(String::from("-"))
                            },
                            child: vec![]
                        },
                        MonitorRow {
                            title: String::from("Fan Speed"),
                            value: None,
                            child: gpu.fans_speed.iter().enumerate().map(|(i, fan_speed)| {
                                MonitorRow {
                                    title: format!("Fan {}", i),
                                    value: Some(format!("{}%", fan_speed)),
                                    child: vec![]
                                }
                            }).collect()
                        },

                    ]
                }
            }).collect()
        }
    }
}