pub mod app;
pub mod window;
pub mod about_window;

const KILO: u16 = 2u16.pow(10);
const MEGA: u32 = 2u32.pow(20);
const GIGA: u32 = 2u32.pow(30);

fn convert_bytes_to_kilo_u32(bytes: u64) -> u32 {
    (bytes / KILO as u64) as u32
}

fn convert_bytes_to_mega_u32(bytes: u64) -> u32 {
    (bytes / MEGA as u64) as u32
}

fn convert_bytes_to_giga_u32(bytes: u64) -> u32 {
    (bytes / GIGA as u64) as u32
}

pub fn get_bytevalue_from(bytes: u64) -> String {
    match bytes {
        1_000 ..= 999_999 => format!("{} KB/s", convert_bytes_to_kilo_u32(bytes)),
        1_000_000 ..= 999_999_999 => format!("{} MB/s", convert_bytes_to_mega_u32(bytes)),
        byte if byte >= 1_000_000_000 => format!("{} GB/s", convert_bytes_to_giga_u32(bytes)),
        _ => format!("{} B/s", bytes as u32)
    }
}