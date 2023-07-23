use gtk::glib;
use gtk::glib::Properties;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::Cell;

// オブジェクトの状態を保持する
#[derive(Default, Properties)]
#[properties(wrapper_type = super::CustomButton)]
pub struct CustomButton {
    #[property(get, set)]
    clicked_count: Cell<u32>,
}

// サブクラスを定義するときの主要なトレイト
#[glib::object_subclass]
impl ObjectSubclass for CustomButton {
    const NAME: &'static str = "MyGtkAppCustomButton";
    type Type = super::CustomButton;
    type ParentType = gtk::Button;
}

// GObjectのサブクラスが実装するトレイト
impl ObjectImpl for CustomButton {
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

// Widgetのサブクラスが実装するトレイト
impl WidgetImpl for CustomButton {}

// Buttonのサブクラスが実装するトレイト
impl ButtonImpl for CustomButton {}
