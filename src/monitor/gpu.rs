use nvml_wrapper::enum_wrappers::device::TemperatureSensor;
use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;

pub struct GpuData {
    name: String,
    fan_speed: u32,
    memory_used: f64,
    memory_available: f64,
    memory_total: f64,
    power_usage: u32,
    temperature: u32,
    usage_gpu: u32,
    usage_memory: u32
}

pub fn get_gpu_data(nvidia: &Result<Nvml, NvmlError>) {
    if let Some(nvidia) = nvidia.as_ref().ok() {
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
            println!(
                "Memory: {} MB / {} MB / {:.1}%",
                memory_info.used / 1000000,
                memory_info.total / 1000000,
                memory_info.used as f64 / memory_info.total as f64 * 100.0
            );
            println!("Power: {} W", power_usage / 1000);
            println!("Temperature: {} C", temperature);
            println!("Usage: {}% (GPU)", utilization.gpu);
            println!("Usage: {}% (MEM)", utilization.memory);
        }
    }
}