extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use std::{thread, time};
use gtk::{Application, Button, SpinButton, Dialog};

/* My wife and I use shutdown +120 a lot to delay shutdown and let shows run while we try to sleep.
   I want to learn some basic Rust, so I'm trying to make a super simple application to provide a "Still There?"
   dialog to opt out of the shutdown. For whatever reason, after I set the timer, my show_input_dialog()
   dialog doesn't disappear when I destroy it @60. I know Python or even bash would be better suited
   for something like this, sorry. */

fn main() {
    let application = Application::new(
        Some("com.github.dipstick.sleeptime_r"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        app.hold();
        // returns response type and a SpinButton value
        let response: (gtk::ResponseType, f64) = show_input_dialog();

        if response.0 == gtk::ResponseType::Accept {
            println!("Start timer for {} minutes.", response.1.to_string());
            sleep_for(response.1);
            show_fallback_dialog();
        } else {
            println!("Cancel!");
        }
        app.release();
    });

    application.run(&[]);
}

fn show_input_dialog() -> (gtk::ResponseType, f64) {
    let dialog = Dialog::new();

    let spinbutton = SpinButton::new_with_range(
        0.0,    // min
        480.0,  // max
        10.0);  // step

    let settimerbutton = Button::new_with_mnemonic(
        "_Set Timer");

    let cancelbutton = Button::new_with_mnemonic(
        "_Cancel");

    dialog.set_title("Set sleep timer?");
    dialog.get_content_area().add(&spinbutton);
    dialog.add_action_widget(&settimerbutton, gtk::ResponseType::Accept);
    dialog.add_action_widget(&cancelbutton, gtk::ResponseType::Cancel);
    dialog.show_all();
    let response = (dialog.run(), spinbutton.get_value());
    dialog.destroy();
    return response;
}

fn show_fallback_dialog() -> gtk::ResponseType {
    let dialog = Dialog::new();
    let label = gtk::Label::new(Some("Are you still there?"));
    let dialogbutton = Button::new_with_mnemonic("_Yes");

    dialog.set_title("Still there?");
    dialog.get_content_area().add(&label);
    dialog.add_action_widget(&dialogbutton, gtk::ResponseType::Yes);
    dialog.show_all();

    return dialog.run();
}

fn sleep_for(minutes: f64) {
    let secondsconv = (minutes * 60.0) as i64;
    let onesecond = time::Duration::new(1,0);
    println!("Sleeping for {} seconds:", secondsconv.to_string());
    for second in 0..secondsconv {
        println!("{}", second.to_string());
        thread::sleep(onesecond);
    }
}
