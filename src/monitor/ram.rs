use sysinfo::{System, SystemExt};

pub struct RamData {
    pub ram_available: f64,
    pub ram_total: f64,
    pub ram_used: f64,
    pub swap_available: f64,
    pub swap_total: f64,
    pub swap_used: f64,
}

impl RamData {
    pub fn new(sys: &System) -> RamData {
        let ram_total = sys.total_memory() as f64;
        let ram_available = sys.free_memory() as f64;
        let ram_used = sys.used_memory() as f64;

        let swap_total = sys.total_swap() as f64;
        let swap_available = sys.free_swap() as f64;
        let swap_used = sys.used_swap() as f64;

        RamData {
            ram_available,
            ram_total,
            ram_used,
            swap_available,
            swap_total,
            swap_used
        }
    }
}