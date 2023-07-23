use crate::{util, Payment};
use gtk;
use gtk::gio;
use gtk::glib::{self, DateTime};
use gtk::prelude::*;

pub fn display_ui(payment: &Payment) -> impl IsA<gtk::Widget> {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .homogeneous(true)
        .build();
    let date = gtk::Label::builder().halign(gtk::Align::Start).build();
    payment
        .bind_property("date", &date, "label")
        .transform_to(|_, d: DateTime| Some(util::format_date(d)))
        .sync_create()
        .build();
    let name = gtk::Label::new(None);
    payment
        .bind_property("name", &name, "label")
        .sync_create()
        .build();
    let amount = gtk::Label::builder().halign(gtk::Align::End).build();
    payment
        .bind_property("amount", &amount, "label")
        .transform_to(|_, a: i64| Some(format!("{}円", a)))
        .sync_create()
        .build();
    hbox.append(&date);
    hbox.append(&name);
    hbox.append(&amount);
    hbox
}

pub fn edit_ui(
    payment: &Payment,
    model: &gio::ListStore,
    row: &gtk::ListBoxRow,
) -> impl IsA<gtk::Widget> {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let name_entry = gtk::Entry::builder()
        .placeholder_text("name")
        .buffer(&gtk::EntryBuffer::builder().text(payment.name()).build())
        .build();
    let amount_entry = gtk::Entry::builder()
        .placeholder_text("amount")
        .buffer(
            &gtk::EntryBuffer::builder()
                .text(format!("{}", payment.amount()))
                .build(),
        )
        .input_purpose(gtk::InputPurpose::Digits)
        .build();
    let (picker, cal) = util::datepicker();
    cal.select_day(&payment.date());
    let update_button = gtk::Button::builder().label("update").build();
    let delete_button = gtk::Button::builder().label("delete").build();

    let index = row.index() as u32;
    update_button.connect_clicked(
        glib::clone!(@weak name_entry, @weak amount_entry, @weak cal, @weak payment, @weak row, @weak model => move |_| {
            let name = name_entry.buffer().text().to_string();
            let amount = match amount_entry.buffer().text().parse::<i64>() {
                Ok(a) => a,
                Err(_) => {
                    util::dialog("エラー", "金額は整数値で入力して下さい");
                    return;
                },
            };
            let date = cal.date();
            // オブジェクトを更新
            payment.set_name(name);
            payment.set_amount(amount);
            payment.set_date(date);
            // ウィジェットを元に戻す
            row.set_child(Some(&display_ui(&payment)));
            // 残高を更新するために items-changed シグナルを発行
            model.items_changed(index, 0, 0);
        }),
    );
    delete_button.connect_clicked(glib::clone!(@weak model => move|_|{
        model.remove(index);
    }));

    hbox.append(&picker);
    hbox.append(&name_entry);
    hbox.append(&amount_entry);
    hbox.append(&update_button);
    hbox.append(&delete_button);
    hbox
}
