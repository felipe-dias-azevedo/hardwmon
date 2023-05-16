use gtk::gdk::keys::constants::S;
use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::{Device, Nvml};

pub struct GpuData {
    name: Option<String>,
    fan_speed: Option<u32>,
    memory_used: Option<u64>,
    memory_available: Option<u64>,
    memory_total: Option<u64>,
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
        let name = device.name().ok();
        let fan_speed = device.fan_speed(0).ok();
        let (memory_used, memory_total, memory_available) = match device.memory_info().ok() {
            Some(mem) => (Some(mem.used), Some(mem.total), Some(mem.free)),
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
            fan_speed,
            memory_used,
            memory_total,
            memory_available,
            power_usage,
            temperature,
            usage_gpu,
            usage_memory,
        }
    }
}