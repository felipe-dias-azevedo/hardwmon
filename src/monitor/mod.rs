use gtk::{glib, prelude::*};

pub fn get_data() -> gtk::TreeStore {
    let store = gtk::TreeStore::new(&[glib::Type::STRING, glib::Type::STRING]);

    // Populate the tree store with some data
    let parent_iter = store.append(None);
    store.set(&parent_iter, &[(0, &"Parent 1".to_value()), (1, &"Value 1".to_value())]);

    let child_iter = store.append(Some(&parent_iter));
    store.set(&child_iter, &[(0, &"Child 1".to_value()), (1, &"Value 2".to_value())]);

    let child_iter = store.append(Some(&parent_iter));
    store.set(&child_iter, &[(0, &"Child 2".to_value()), (1, &"Value 3".to_value())]);

    let child_iter = store.append(Some(&parent_iter));
    store.set(&child_iter, &[(0, &"Child 3".to_value()), (1, &"Value 4".to_value())]);

    let parent_iter = store.append(None);
    store.set(&parent_iter, &[(0, &"Parent 2".to_value()), (1, &"Value 5".to_value())]);

    let child_iter = store.append(Some(&parent_iter));
    store.set(&child_iter, &[(0, &"Child 4".to_value()), (1, &"Value 6".to_value())]);

    let child_iter = store.append(Some(&parent_iter));
    store.set(&child_iter, &[(0, &"Child 5".to_value()), (1, &"Value 7".to_value())]);

    return store;
}