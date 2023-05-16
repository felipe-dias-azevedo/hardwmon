use sysinfo::{CpuExt, System, SystemExt};

pub struct CpuData {
    pub brand: String,
    pub usage_total: f32,
    pub frequency_total: u64,
    pub usage: Vec<f32>,
    pub frequency: Vec<u64>
}

pub fn get_cpu_data(sys: &System) {
    {
        println!("------ CPU -----");
        let cpus = sys.cpus();
        let brand = sys.global_cpu_info().brand();
        // let freq = sys.global_cpu_info().frequency();
        // let usage = sys.global_cpu_info().cpu_usage();

        println!("Name: {}", brand);

        for i in 0..cpus.len() {
            let cpu = &cpus[i];

            let freq = cpu.frequency();
            let usage = cpu.cpu_usage();

            println!("[CPU {}] Frequency: {} MHz", i, freq);
            println!("[CPU {}] Usage: {:.0}%", i, usage);
            println!();
        }
    }
}