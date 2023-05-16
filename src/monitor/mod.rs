mod cpu;
mod gpu;
mod sensors;
mod ram;
mod network;
mod system;
mod disk;

use gtk::{glib, prelude::*};

pub struct MonitorRow {
    title: String,
    value: Option<String>,
    child: Vec<MonitorRow>,
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

pub fn get_tree_model() -> gtk::TreeStore {
    let store = gtk::TreeStore::new(&[glib::Type::STRING, glib::Type::STRING]);

    let data = simulate_data();

    populate_data(&store, data, None);

    return store;
}