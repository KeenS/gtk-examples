use gtk::gio;
use gtk::glib;
use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

mod payment;
mod ui;
mod util;
use payment::Payment;

#[derive(Debug)]
pub struct Wado {
    model: gio::ListStore,
    balance: gtk::Label,
}

impl Default for Wado {
    fn default() -> Self {
        let label = gtk::Label::new(None);
        let model = gio::ListStore::new(Payment::static_type());
        model.connect_items_changed(glib::clone!(@weak label => move |m, _, _, _| {
            let balance = m
                .into_iter()
                .map(|item| item.unwrap().downcast::<Payment>().unwrap().amount())
                .sum::<i64>();
            label.set_markup(&format!("<big>{}</big>円", balance));
        }));
        Self {
            model,
            balance: label,
        }
    }
}

impl Wado {
    pub fn record_payment(&mut self, payment: Payment) {
        self.model.append(&payment);
    }

    pub fn model(&self) -> &gio::ListStore {
        &self.model
    }
    pub fn balance(&self) -> &gtk::Label {
        &self.balance
    }
}

fn main() -> glib::ExitCode {
    let application = gtk::Application::new(
        Some("com.github.keens.gtk-examples.wado"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(app: &gtk::Application) {
    // See https://github.com/gtk-rs/gtk4-rs/blob/master/examples/list_box_model/main.rs
    let window = gtk::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("ワドー")
        .build();

    let mut wado = Wado::default();

    wado.record_payment(Payment::new(
        "お小遣い".into(),
        1000,
        glib::DateTime::now_local().unwrap(),
    ));
    wado.record_payment(Payment::new(
        "きゅうり".into(),
        -150,
        glib::DateTime::now_local().unwrap(),
    ));

    let list_box = gtk::ListBox::new();
    list_box.bind_model(Some(wado.model()), |item| {
        let payment = item.downcast_ref::<Payment>().unwrap();
        ui::display_ui(payment).upcast::<gtk::Widget>()
    });

    let model = wado.model();
    list_box.connect_row_activated(glib::clone!(@weak model => move |_lbox, row| {
        let payment = model.item(row.index() as u32).unwrap().downcast::<Payment>().unwrap();

        row.set_child(Some(&ui::edit_ui(&payment, &model, row)));
    }));

    let scrolled_window = gtk::ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
        .min_content_height(400)
        .child(&list_box)
        .build();

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(10)
        .margin_bottom(10)
        .margin_end(10)
        .margin_start(10)
        .margin_top(10)
        .build();

    let wado = Rc::new(RefCell::new(wado));

    let frame = gtk::Frame::builder()
        .label("残高")
        .child(wado.borrow_mut().balance())
        .build();
    vbox.append(&frame);
    vbox.append(&scrolled_window);
    vbox.append(&input_box(wado));
    window.set_child(Some(&vbox));
    window.show();
}

fn input_box(wado: Rc<RefCell<Wado>>) -> gtk::Box {
    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();
    let name = gtk::Entry::builder().placeholder_text("name").build();
    let amount = gtk::Entry::builder()
        .placeholder_text("amount")
        .input_purpose(gtk::InputPurpose::Digits)
        .build();
    let (picker, cal) = util::datepicker();
    let new_button = gtk::Button::builder().label("new").build();

    new_button.connect_clicked(
        glib::clone!(@weak name, @weak amount, @weak cal, @strong wado => move |_| {
            let n = name.buffer().text().to_string();
            let amount = match amount.buffer().text().parse() {
                Ok(a) => a,
                Err(_) => {
                    util::dialog("エラー", "金額は整数値で入力して下さい");
                    return;
                },
            };
            let date = cal.date();
            let payment = Payment::new(n, amount, date);
            wado.borrow_mut().record_payment(payment);
        }),
    );

    // 並びは出入金リストと同じく日付、名前、金額の順
    hbox.append(&picker);
    hbox.append(&name);
    hbox.append(&amount);
    hbox.append(&new_button);
    hbox
}
