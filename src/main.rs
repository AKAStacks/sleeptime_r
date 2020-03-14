extern crate clap;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate system_shutdown;

use gio::prelude::*;
use gtk::prelude::*;
use shrinkwraprs::*;

use clap::{App, Arg};
use glib::source::source_remove;
use gtk::{Application, Button, Dialog, ResponseType, SpinButton};
use std::{thread, time};
use system_shutdown::shutdown;

fn main() {
    // We're setting up optional CLI arguments here.
    let args = App::new("sleeptime_r")
        .version("0.1.0")
        .author("Some dummy named Dennis Zarger")
        .about("GTK+ application for setting a sleep timer for your PC.")
        .arg(
            Arg::with_name("still there length")
                .short("s")
                .long("stillthere")
                .value_name("INTEGER")
                .help("Sets timeout of 'still there' window (in seconds). Default: 10, Max: 255")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("default time")
                .short("d")
                .long("default")
                .value_name("INTEGER")
                .help("Sets initial value of timer (in minutes). Default: 0, Max: 480")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Enables verbose print statements."),
        )
        .get_matches();

    // We're parsing those arguments here
    let mut still_there_time = args
        .value_of("still there length")
        .unwrap_or("10")
        .parse::<u8>()
        .unwrap();

    let default_timer_length = args
        .value_of("default time")
        .unwrap_or("120")
        .parse::<f64>()
        .unwrap();

    let verbose = args.is_present("v");

    // Now we're going into the application
    let application = Application::new(Some("com.github.dipstick.sleeptime_r"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(move |app| {
        app.hold();
        let (response_type, sleep_time) = show_sleep_time_dialog(default_timer_length);

        if response_type == ResponseType::Accept {
            if verbose {
                println!("Start timer for {} minutes.", &sleep_time.to_string());
            }
            sleep_for(&sleep_time, verbose);

            let timeout_source = gtk::timeout_add_seconds(1, move || {
                still_there_time -= 1;
                match still_there_time {
                    1..=255 => {
                        if verbose {
                            println!("Shutdown in: {}", still_there_time.to_string())
                        };
                        Continue(true)
                    }
                    0 => {
                        try_shutdown(verbose);
                        Continue(false)
                    }
                }
            });
            if show_fallback_dialog() == ResponseType::Yes {
                if verbose {
                    println!("Cancelling shutdown!");
                }
                source_remove(timeout_source);
            }
        } else {
            if verbose {
                println!("Cancel!");
            }
        }
        app.release();
    });

    application.run(&[]);
}

#[derive(Shrinkwrap)]
struct SleepTimeDialog {
    spinbutton: SpinButton,
    #[shrinkwrap(main_field)]
    dialog: Dialog,
}

impl SleepTimeDialog {
    fn new(title: &str, min: f64, max: f64, step: f64, default_timer_length: f64) -> SleepTimeDialog {
        let dialog = Dialog::new();
        let spinbutton = SpinButton::new_with_range(min, max, step);
        let cancelbutton = Button::new_with_mnemonic("_Cancel");
        let cancelbutton_clone = cancelbutton.clone();

        spinbutton.connect_activate(move |_| {
            cancelbutton_clone.set_label("Timer Set!");
        });
        spinbutton.set_value(default_timer_length);

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
        let response_type: ResponseType = self.dialog.run();
        let value = self.get_value();
        unsafe {
            self.destroy();
        };
        (response_type, value)
    }
}

fn show_sleep_time_dialog(default_timer_length: f64) -> (ResponseType, f64) {
    let dialog = SleepTimeDialog::new("Set sleep timer?", 0.0, 480.0, 5.0, default_timer_length);
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

fn sleep_for(minutes: &f64, verbose: bool) {
    let secondsconv = (minutes * 60.0) as u64;
    let sleep_time = time::Duration::new(secondsconv, 0);
    if verbose {
        println!("Sleeping for {} seconds:", &secondsconv.to_string())
    };
    loop {
        if gtk::events_pending() {
            gtk::main_iteration_do(false);
        } else {
            thread::sleep(sleep_time);
            break;
        }
    }
}

fn try_shutdown(verbose: bool) {
    match shutdown() {
        Ok(_) => {
            if verbose {
                println!("Shutting down.")
            }
        }
        Err(error) => println! {"Couldn't shut down, {}", error},
    }
}
