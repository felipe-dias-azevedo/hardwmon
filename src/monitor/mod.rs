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

pub struct MonitorData {
    pub data: Vec<MonitorRow>
}

#[derive(Clone)]
pub struct MonitorRow {
    pub title: String,
    pub value: Option<String>,
    pub child: Vec<MonitorRow>,
}

impl MonitorRow {
    fn to_single_child(&self) -> Vec<MonitorRow> {
        let mut elements = Vec::new();
        for c in &self.child {
            if !c.child.is_empty() {
                elements.append(&mut c.to_single_child());
            } else {
                elements.push(c.clone());
            }
        }
        elements
    }

    pub fn on_single_list(data: Vec<MonitorRow>) -> Vec<MonitorRow> {
        let mut elements = Vec::new();
        for r in data {
            if !r.child.is_empty() {
                elements.append(&mut r.to_single_child());
            } else {
                elements.push(r.clone());
            }
        }
        elements
    }
}

pub fn get_hardware_data(sys: &System, nvidia: &Result<Nvml, NvmlError>) -> Vec<MonitorRow> {
    let cpu = cpu::CpuData::new(sys).format();
    let ram = ram::RamData::new(sys).format();
    let disk = disk::DisksData::new(sys).format();
    let network = network::NetworkData::format(network::NetworkData::new(sys));
    let gpu = gpu::GpuData::format(gpu::GpuData::new(nvidia));
    let sensors = sensors::SensorData::format(sensors::SensorData::new());
    let system = system::SystemData::new(sys);

    vec![cpu, ram, disk, network, sensors, gpu]
}

pub fn populate_data(store: &gtk::TreeStore, data: Vec<MonitorRow>, old_iter: Option<&gtk::TreeIter>) {

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