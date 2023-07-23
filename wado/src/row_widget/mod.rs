mod imp;
use crate::Payment;
use gtk::glib;

glib::wrapper! {
    pub struct RowWidget(ObjectSubclass<imp::RowWidget>)
        @extends gtk::Widget, gtk::ListBoxRow;
}

impl RowWidget {
    pub fn new(payment: &Payment) -> Self {
        glib::Object::builder()
            .property("row-data", &payment)
            .build()
    }
}
