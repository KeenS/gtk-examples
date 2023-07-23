use std::{cell::Cell, rc::Rc};

use gtk::{glib, prelude::*, Application, ApplicationWindow};

const APP_ID: &str = "com.github.keens.gtk-examples.memory-management";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(application: &Application) {
    // ボタンを2つ作る
    let button_increase = gtk::Button::builder()
        .label("Increase")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let button_decrease = gtk::Button::builder()
        .label("Decrease")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    // 2つのボタンから変更する数字を用意する
    let number = Rc::new(Cell::new(0));

    // コールバックを接続する
    // ボタンをクリックすると `number` ともう1つのボタンのラベルを変える

    button_increase.connect_clicked(glib::clone!(@strong number, @weak button_decrease =>
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
    }));
    button_decrease.connect_clicked(glib::clone!(@weak button_increase =>
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
    }));

    // ボタンをgtk_boxに追加する
    let gtk_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);

    // ウィンドウを作る
    let window = ApplicationWindow::builder()
        .application(application)
        .title("My GTK App")
        .child(&gtk_box)
        .build();

    // ウィンドウを表示する
    window.present();
}
