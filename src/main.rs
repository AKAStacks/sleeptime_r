extern crate gtk;
extern crate gio;
extern crate glib;
extern crate system_shutdown;

use gtk::prelude::*;
use gio::prelude::*;
use shrinkwraprs::*;

use system_shutdown::shutdown;
use std::{thread, time};
use gtk::{Application, Button, SpinButton, Dialog, ResponseType};
use glib::source::source_remove;

fn main() {
    let application = Application::new(
        Some("com.github.dipstick.sleeptime_r"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        app.hold();
        // returns response type and a SpinButton value
        let (responsetype, sleeptime) = show_sleep_time_dialog();

        if responsetype == ResponseType::Accept {
            println!("Start timer for {} minutes.", &sleeptime.to_string());
            sleep_for(&sleeptime);

            let mut timeout: u8 = 10;

            let timeout_source = gtk::timeout_add_seconds(1, move || {
                timeout -= 1;
                match timeout {
                    1..=10 => {
                        println!("Shutdown in: {}", timeout.to_string());
                        Continue(true)
                    },
                    0 => {
                        match shutdown() {
                            Ok(_) => println!{"Shutting down."},
                            Err(error) => println!{"Couldn't shut down, {}", error},
                        }
                        Continue(false)
                    },
                    _ => Continue(true),
                }
            });
            if show_fallback_dialog() == ResponseType::Yes {
                println!("Cancelling shutdown!");
                source_remove(timeout_source);
            }
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
        dialog.add_action_widget(&spinbutton, ResponseType::Accept);
        dialog.add_action_widget(&cancelbutton, ResponseType::Cancel);
        dialog.show_all();
        SleepTimeDialog { dialog, spinbutton }
    }

    fn get_value(&self) -> f64 {
        self.spinbutton.get_value()
    }

    fn run(&self) -> (ResponseType, f64) {
        let responsetype: ResponseType = self.dialog.run();
        let value = self.get_value();
        unsafe {
            self.destroy();
        };
        (responsetype, value)
    }
}

fn show_sleep_time_dialog() -> (ResponseType, f64) {
    let dialog = SleepTimeDialog::new(
        "Set sleep timer?",
        0.0,
        480.0,
        5.0,
    );
    let response = dialog.run();

    return response;
}

fn show_fallback_dialog() -> ResponseType {
    let dialog = Dialog::new();
    let label = gtk::Label::new(Some("Are you still there?"));
    let dialogbutton = Button::new_with_mnemonic("_Yes");

    dialog.set_title("Still there?");
    dialog.get_content_area().add(&label);
    dialog.add_action_widget(&dialogbutton, ResponseType::Yes);
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
