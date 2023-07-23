use crate::util;
use std::cell::RefCell;

use gtk::{
    glib::{self, clone, DateTime, ParamSpec, Properties, Value},
    prelude::*,
    subclass::prelude::*,
    ResponseType,
};

use crate::Payment;

#[derive(Default, Properties, Debug)]
#[properties(wrapper_type = super::RowWidget)]
pub struct RowWidget {
    #[property(get, set, construct_only)]
    row_data: RefCell<Option<Payment>>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowWidget {
    const NAME: &'static str = "RowWidget";
    type ParentType = gtk::ListBoxRow;
    type Type = super::RowWidget;
}

impl ObjectImpl for RowWidget {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }

    fn constructed(&self) {
        self.parent_constructed();
        let obj = self.obj();

        let item = self.row_data.borrow();
        let item = item.as_ref().cloned().unwrap();

        let b = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(10)
            .vexpand(true)
            .homogeneous(true)
            .build();
        let date = gtk::Label::builder().halign(gtk::Align::Start).build();
        item.bind_property("date", &date, "label")
            .transform_to(|_, d: DateTime| Some(util::format_date(d)))
            .sync_create()
            .build();
        let name = gtk::Label::new(None);
        item.bind_property("name", &name, "label")
            .sync_create()
            .build();
        let amount = gtk::Label::builder().halign(gtk::Align::End).build();
        item.bind_property("amount", &amount, "label")
            .transform_to(|_, a: i64| Some(format!("{}円", a)))
            .sync_create()
            .build();
        b.append(&date);
        b.append(&name);
        b.append(&amount);

        obj.set_child(Some(&b));
    }
}

impl WidgetImpl for RowWidget {}
impl ListBoxRowImpl for RowWidget {}
