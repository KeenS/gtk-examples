mod imp;
use gtk;
use gtk::glib::{self, DateTime};

glib::wrapper! {
    pub struct Payment(ObjectSubclass<imp::Payment>);
}

impl Payment {
    pub fn new(name: String, amount: i64, date: DateTime) -> Self {
        let obj = glib::Object::new::<Payment>();
        obj.set_name(name);
        obj.set_amount(amount);
        obj.set_date(date);
        obj
    }
}
