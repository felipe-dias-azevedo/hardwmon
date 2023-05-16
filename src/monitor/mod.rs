mod cpu;
mod gpu;
mod sensors;
mod ram;
mod network;
mod system;
mod disk;

use gtk::{glib, prelude::*};
use nvml_wrapper::{Nvml, error::NvmlError};
use sysinfo::System;

pub struct MonitorRow {
    title: String,
    value: Option<String>,
    child: Vec<MonitorRow>,
}

pub fn get_hardware_data(sys: &System, nvidia: &Result<Nvml, NvmlError>) -> Vec<MonitorRow> {
    let cpu = cpu::CpuData::new(sys);
    let disk = disk::DisksData::new(sys);
    let gpu = gpu::GpuData::new(nvidia);
    let network = network::NetworkData::new(sys);
    let ram = ram::RamData::new(sys);
    let sensors = sensors::SensorData::new();
    let system = system::SystemData::new(sys);

    vec![]
}

fn simulate_data() -> Vec<MonitorRow> {
    vec![
        MonitorRow {
            title: String::from("CPU"),
            value: None,
            child: vec![
                MonitorRow {
                    title: String::from("Usage"),
                    value: None,
                    child: vec![
                        MonitorRow {
                            child: vec![],
                            title: String::from("0 Core"),
                            value: Some(String::from("30%")),
                        },
                        MonitorRow {
                            child: vec![],
                            title: String::from("1 Core"),
                            value: Some(String::from("2%")),
                        },
                    ],
                },
                MonitorRow {
                    child: vec![],
                    title: String::from("Frequency"),
                    value: Some(String::from("4600 MHz")),
                },
            ],
        },
        MonitorRow {
            title: String::from("GPU"),
            value: None,
            child: vec![
                MonitorRow {
                    child: vec![],
                    title: String::from("Usage"),
                    value: Some(String::from("30%")),
                },
                MonitorRow {
                    child: vec![],
                    title: String::from("Power"),
                    value: Some(String::from("46 W")),
                },
            ],
        },
    ]
}

fn populate_data(store: &gtk::TreeStore, data: Vec<MonitorRow>, old_iter: Option<&gtk::TreeIter>) {

    for r in data {

        let iter = store.append(old_iter);
        let title = r.title.to_value();
        let value = r.value.to_value();

        store.set(&iter, &[(0, &title), (1, &value)]);

        if !r.child.is_empty() {
            populate_data(store, r.child, Some(&iter));
        }
    }
}

pub fn get_tree_model(data: Vec<MonitorRow>) -> gtk::TreeStore {
    let store = gtk::TreeStore::new(&[glib::Type::STRING, glib::Type::STRING]);

    populate_data(&store, data, None);

    return store;
}