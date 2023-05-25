use crate::views::about_window::AboutWindow;
use gtk::glib::{clone, PropertyGet};
use gtk::prelude::*;
use crate::monitor;
use crate::monitor::MonitorRow;

pub struct Window {
    pub window: gtk::Window,
    menupopover: gtk::Popover,
    aboutbutton: gtk::Button,
    preferencesbutton: gtk::Button,
    headerpaned: gtk::Paned,
    contentpaned: gtk::Paned,
    subheader: gtk::HeaderBar,
    subcontent: gtk::Box,
    headerpanelbutton: gtk::Button,
    maintreeview: gtk::TreeView,
}

impl Window {
    pub fn new(builder: &gtk::Builder) -> Self {
        let main_window = Window {
            window: builder.object("appwindow").expect("Couldn't set window"),
            menupopover: builder
                .object("menupopover")
                .expect("Couldn't get menupopover"),
            aboutbutton: builder
                .object("aboutbutton")
                .expect("Couldn't get aboutbutton"),
            preferencesbutton: builder
                .object("preferencesbutton")
                .expect("Couldn't get preferencesbutton"),
            headerpaned: builder
                .object("headerpaned")
                .expect("Couldn't get headerpaned"),
            contentpaned: builder
                .object("contentpaned")
                .expect("Couldn't get contentpaned"),
            subheader: builder.object("subheader").expect("Couldn't get subheader"),
            subcontent: builder
                .object("subcontent")
                .expect("Couldn't get subcontent"),
            headerpanelbutton: builder
                .object("headerpanelbutton")
                .expect("Couldn't get headerpanelbutton"),
            maintreeview: builder
                .object("maintreeview")
                .expect("Couldn't get maintreeview"),
        };

        main_window.window.show_all();

        main_window.reset_components();

        main_window
    }

    fn reset_components(&self) {
        self.preferencesbutton.set_sensitive(false);

        self.subheader.set_visible(false);
        self.subcontent.set_visible(false);

        self.headerpanelbutton.set_sensitive(false);
    }

    pub fn on_close(&self) {
        self.window.connect_delete_event(|_, _| {
            // gtk::main_quit();

            Inhibit(false)
        });
    }

    pub fn on_about_clicked(&self, builder: &gtk::Builder) {
        self.aboutbutton.connect_clicked(clone!(
            @strong self.menupopover as menupopover,
            @strong builder
            => move |_| {
                let _ = AboutWindow::new(&builder);

                menupopover.hide();
        }));
    }

    pub fn on_sidebar_toggle_clicked(&self) {
        self.headerpanelbutton.connect_clicked(clone!(@strong self.subheader as subheader, @strong self.subcontent as subcontent => move |_| {
            subheader.set_visible(!subheader.is_visible());
            subcontent.set_visible(!subcontent.is_visible());
        }));
    }

    pub fn set_treeview(&self, tree_model: gtk::TreeStore) {
        self.maintreeview.set_model(Some(&tree_model));
        self.maintreeview.expand_all();

        // Define the columns
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::TreeViewColumnExt::pack_start(&column, &renderer, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&column, &renderer, "text", 0);
        column.set_resizable(true);
        column.set_sizing(gtk::TreeViewColumnSizing::GrowOnly);
        column.set_min_width(150);
        column.set_title("Name");
        self.maintreeview.append_column(&column);

        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        gtk::prelude::TreeViewColumnExt::pack_start(&column, &renderer, true);
        gtk::prelude::TreeViewColumnExt::add_attribute(&column, &renderer, "text", 1);
        column.set_resizable(true);
        column.set_sizing(gtk::TreeViewColumnSizing::GrowOnly);
        column.set_min_width(150);
        column.set_title("Value");
        self.maintreeview.append_column(&column);
    }

    fn iter_tree(old_tree_model: &gtk::TreeStore, iter: &gtk::TreeIter, data: &Vec<MonitorRow>) -> bool {
        let value = old_tree_model.value(&iter, 0).get::<String>().unwrap();

        let row = data.iter().find(|&x| x.title == value);

        if let Some(row) = row {
            old_tree_model.set_value(&iter, 1, &row.value.to_value());
        } else {
            old_tree_model.set_value(&iter, 1, &"-".to_value());
        }

        !old_tree_model.iter_next(&iter)
    }

    fn iter_tree_child(old_tree_model: &gtk::TreeStore, iter: &gtk::TreeIter, data: &Vec<MonitorRow>) {
        let mut done = false;

        while !done {
            if !old_tree_model.iter_has_child(&iter) {
                done = Self::iter_tree(&old_tree_model, &iter, data);
            } else {
                let child_iter = old_tree_model.iter_nth_child(Some(&iter), 0).unwrap();
                Self::iter_tree_child(&old_tree_model, &child_iter, data);
                done = !old_tree_model.iter_next(&iter);
            }
        }
    }

    pub fn update_treeview(&self, data: Vec<MonitorRow>) {
        if let Some(old_tree_model) = self.maintreeview.model() {
            let old_tree_model = old_tree_model.downcast::<gtk::TreeStore>().unwrap();

            let iter = old_tree_model.iter_first();

            if let Some(iter) = iter {
                Self::iter_tree_child(&old_tree_model, &iter, &data);
            }
        }
    }
}
