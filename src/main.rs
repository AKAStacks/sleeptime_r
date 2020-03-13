extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use shrinkwraprs::*;

use std::{thread, time};
use gtk::{Application, Button, SpinButton, Dialog};

/* My wife and I use shutdown +120 a lot to delay shutdown and let shows run while we try to sleep.
   I want to learn some basic Rust, so I'm trying to make a super simple application to provide a "Still There?"
   dialog to opt out of the shutdown. For whatever reason, after I set the timer, my show_sleep_time_dialog()
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
        let response: (gtk::ResponseType, f64) = show_sleep_time_dialog();
        let responsetype: gtk::ResponseType = response.0.clone();
        let sleeptime: f64 = response.1.clone();

        if responsetype == gtk::ResponseType::Accept {
            println!("Start timer for {} minutes.", &sleeptime.to_string());
            sleep_for(&sleeptime);
            show_fallback_dialog();
        } else {
            println!("Cancel!");
        }
        app.release();
    });

    application.run(&[]);
}

#[derive(Shrinkwrap)]
struct SleepTimeDialog {
    spinbutton: SpinButton,
    #[shrinkwrap(main_field)] dialog: Dialog,
}

impl SleepTimeDialog {
    fn new(title: &str, min: f64, max: f64, step: f64) -> SleepTimeDialog {
        let dialog = Dialog::new();
        let spinbutton = SpinButton::new_with_range(
            min,
            max,
            step
        );
        let cancelbutton = Button::new_with_mnemonic("_Cancel");
        let cancelbutton_clone = cancelbutton.clone();

        spinbutton.connect_activate(move |_| {
            cancelbutton_clone.set_label("Timer Set!");
        });

        dialog.set_title(title);
        dialog.add_action_widget(&spinbutton, gtk::ResponseType::Accept);
        dialog.add_action_widget(&cancelbutton, gtk::ResponseType::Cancel);
        dialog.show_all();
        SleepTimeDialog { dialog, spinbutton }
    }

    fn get_value(&self) -> f64 {
        self.spinbutton.get_value()
    }

    fn run(&self) -> (gtk::ResponseType, f64) {
        let responsetype: gtk::ResponseType = self.dialog.run();
        let value = self.get_value();
        self.destroy();
        (responsetype, value)
    }
}

fn show_sleep_time_dialog() -> (gtk::ResponseType, f64) {
    let dialog = SleepTimeDialog::new(
        "Set sleep timer?",
        0.0,
        480.0,
        5.0,
    );
    let response = dialog.run();

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

    dialog.run()
}

fn sleep_for(minutes: &f64) {
    let secondsconv = (minutes * 60.0) as u64;
    let sleeptime = time::Duration::new(secondsconv,0);
    println!("Sleeping for {} seconds:", &secondsconv.to_string());
    loop {
        if gtk::events_pending() {
            gtk::main_iteration_do(false);
        } else {
            thread::sleep(sleeptime);
            break;
        }
    }
}
