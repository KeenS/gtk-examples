use gtk::glib;
use gtk::prelude::*;

pub fn format_date(d: glib::DateTime) -> String {
    format!("{:04}-{:02}-{:02}", d.year(), d.month(), d.day_of_month())
}

pub fn datepicker() -> (gtk::Box, gtk::Calendar) {
    let hbox = gtk::Box::builder()
        .homogeneous(false)
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let button = gtk::Button::new();
    let cal = gtk::Calendar::new();
    // デフォルトで今日の日付を選択しておく
    cal.select_day(&glib::DateTime::now_local().unwrap());
    let pop = gtk::Popover::builder().child(&cal).autohide(true).build();

    button.connect_clicked(glib::clone!(@weak pop => move |_| {
        pop.popup()
    }));
    button.set_label(&format_date(cal.date()));
    cal.connect_day_selected(glib::clone!(@weak pop, @weak button => move |cal| {
        pop.popdown();
        // 日付が選ばれたらボタンのラベルに反映しておく
        button.set_label(&format_date(cal.date()));
    }));

    hbox.append(&button);
    hbox.append(&pop);

    // 親ウィジェットに追加するためのBoxと
    // 選択された日付を取り出すためのカレンダーを返す
    (hbox, cal)
}

pub fn dialog(title: &str, message: &str) {
    let label = gtk::Label::new(Some(message));
    let button = gtk::Button::with_label("OK");
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .build();
    vbox.append(&label);
    vbox.append(&button);
    let dialog = gtk::Window::builder().title(title).child(&vbox).build();
    button.connect_clicked(glib::clone!(@weak dialog => move |_| {
        dialog.close()
    }));
    dialog.show();
}
