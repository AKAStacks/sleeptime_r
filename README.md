# sleeptime_r

Sleeptime_r is a simple GTK+ application used to set a sleep timer for your PC (like the one on your television).

Thanks to the authors of the shrinkwraprs, clap, and system_shutdown crates.

It's a bit like using ```bash shutdown +m``` except the +m value is taken via GUI. At the scheduled time, a second window will appear to allow the user to opt-out of the scheduled shutdown (if you can't sleep--like me.)

## Installation

TBD: Need PKGBUILD and .deb

## Usage

```bash sleeptime_r [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v               Enables verbose print statements.
    -V, --version    Prints version information

OPTIONS:
    -d, --default <INTEGER>       Sets initial value of timer (in minutes). Default: 0, Max: 480
    -s, --stillthere <INTEGER>    Sets timeout of 'still there' window (in seconds). Default: 10, Max: 255```

Please note that 0 is a valid value for the delay. This allows for testing of the above options, or if you just want to shutdown after the default 10 seconds.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
