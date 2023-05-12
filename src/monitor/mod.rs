use gtk::{glib, prelude::*};

// pub fn get_data() -> gtk::TreeStore {
//     let store = gtk::TreeStore::new(&[glib::Type::STRING, glib::Type::STRING]);
//
//     // Populate the tree store with some data
//     let parent_iter = store.append(None);
//     store.set(&parent_iter, &[0, 1], &["Parent 1".to_value(), "Value 1".to_value()]);
//
//     let child_iter = store.append(Some(&parent_iter));
//     store.set(&child_iter, &[0, 1], &["Child 1".to_value(), "Value 2".to_value()]);
//
//     let child_iter = store.append(Some(&parent_iter));
//     store.set(&child_iter, &[0, 1], &["Child 2".to_value(), "Value 3".to_value()]);
//
//     let child_iter = store.append(Some(&parent_iter));
//     store.set(&child_iter, &[0, 1], &["Child 3".to_value(), "Value 4".to_value()]);
//
//     let parent_iter = store.append(None);
//     store.set(&parent_iter, &[0, 1], &["Parent 2".to_value(), "Value 5".to_value()]);
//
//     let child_iter = store.append(Some(&parent_iter));
//     store.set(&child_iter, &[0, 1], &["Child 4".to_value(), "Value 6".to_value()]);
//
//     let child_iter = store.append(Some(&parent_iter));
//     store.set(&child_iter, &[0, 1], &["Child 5".to_value(), "Value 7".to_value()]);
//
//     return store;
// }