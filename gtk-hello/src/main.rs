// 1
use gtk::prelude::*;

fn main() {
    // 2
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());
    // 3
    application.connect_activate(build_ui);
    // 4
    application.run();
}

fn build_ui(application: &gtk::Application) {
    // 5
    let window = gtk::ApplicationWindow::new(application);

    // 6
    window.set_title(Some("First GTK Program"));
    window.set_default_size(350, 70);

    // 7
    let button = gtk::Button::with_label("Click me!");
    // 8
    button.connect_clicked(|_| {
        println!("Clicked!");
    });

    // 9
    window.set_child(Some(&button));

    // 10
    window.show();
}
