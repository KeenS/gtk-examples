use gtk::glib;
use gtk::glib::{prelude::*, DateTime, Properties};
use gtk::subclass::prelude::*;
use std::cell::{Cell, RefCell};

#[derive(Debug, Properties)]
#[properties(wrapper=super::Payment)]
pub struct Payment {
    #[property(get, set)]
    name: RefCell<String>,
    #[property(get, set)]
    amount: Cell<i64>,
    #[property(get, set)]
    date: RefCell<DateTime>,
}

impl Default for Payment {
    // for object_subclass
    fn default() -> Self {
        Self {
            name: RefCell::new(String::new()),
            amount: Cell::new(0),
            date: RefCell::new(DateTime::now_local().unwrap()),
        }
    }
}

#[glib::object_subclass]
impl ObjectSubclass for Payment {
    const NAME: &'static str = "Payment";
    type Type = super::Payment;
    type ParentType = glib::Object;
}

impl ObjectImpl for Payment {
    fn properties() -> &'static [glib::ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        self.derived_property(id, pspec)
    }
}
