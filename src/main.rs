extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button, Entry, Box};

fn main() {
    let application = Application::new(  // Prototype gtk's Application type
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350,70);

        let vbox = Box::new(gtk::Orientation::Vertical, 2);

        let entry = Entry::new();
        let entry_clone = entry.clone();

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(move |_| {
            let entry_val = entry_clone.get_buffer().get_text();
            println!("{}", entry_val);
        });

        vbox.add(&button);
        vbox.add(&entry);
        window.add(&vbox);
        window.show_all();
    });

    application.run(&[]);
}
