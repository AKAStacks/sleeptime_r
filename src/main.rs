extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::{thread, time};
use gtk::{Application, Button, SpinButton, Dialog};

fn main() {
    let application = Application::new(  // Prototype gtk's Application type
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        app.hold();
        let dialog = Dialog::new();
        let dialog_content_area = dialog.get_content_area();

        let spinbutton = SpinButton::new_with_range(
            0.0,    // min
            480.0,  // max
            10.0);  // step
        let spinbutton_clone = spinbutton.clone();

        let settiimerbutton = Button::new_with_mnemonic(
            "_Set Timer");

        let cancelbutton = Button:: new_with_mnemonic(
            "_Cancel")

        settiimerbutton.connect_clicked(move |_| {
            let sleepmins: f64 = spinbutton_clone.get_value();
            //application_clone.hold();
            dialog.close();
            sleep_for(sleepmins);
            println!("Slept for {} mins!", sleepmins);
            app.release();
        });

        dialog_content_area.add(&spinbutton);
        dialog.add_action_widget(&settiimerbutton, gtk::ResponseType::Accept);
        dialog.add_action_widget(&cancelbutton, gtk::ResponseType::Cancel);
        dialog.show_all();
        dialog.run();
        //window.add(&vbox);
        //window.show_all();
    });

    application.run(&[]);
}

fn show_input_dialog(app: Application) {
}

fn sleep_for(minutes: f64) {
    let secondsconv = minutes * 60.0;
    let secondsint = secondsconv as i64;
    let onesecond = time::Duration::new(1,0);
    println!("Sleeping for {} seconds:", secondsint.to_string());
    for second in 0..secondsint {
        println!("{}", second.to_string());
        thread::sleep(onesecond);
    }
}
