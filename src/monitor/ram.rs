use sysinfo::{System, SystemExt};

pub struct RamData {
    pub ram_available: f64,
    pub ram_total: f64,
    pub ram_used: f64,
    pub swap_available: f64,
    pub swap_total: f64,
    pub swap_used: f64
}

pub fn get_ram_data(sys: &System) {
    {
        println!("------ RAM -----");
        let total = sys.total_memory() as f64;
        let available = sys.free_memory() as f64;
        let used = sys.used_memory() as f64;

        println!(
            "Usage: {:.2} GB / {:.2} GB / {:.2} %",
            used / 1_000_000_000.0,
            total / 1_000_000_000.0,
            (used / total) * 100.0
        );
        println!(
            "Available: {:.2} GB / {:.2} GB / {:.2} %",
            available / 1_000_000_000.0,
            total / 1_000_000_000.0,
            ((total - available) / total) * 100.0
        )
    }

    {
        println!("------ SWAP -----");
        let total = sys.total_swap() as f64;
        let available = sys.free_swap() as f64;
        let used = sys.used_swap() as f64;

        println!(
            "Usage: {:.2} GB / {:.2} GB / {:.2} %",
            used / 1_000_000_000.0,
            total / 1_000_000_000.0,
            (used / total) * 100.0
        );
        println!("Available: {:.2} GB", available / 1_000_000_000.0)
    }
}