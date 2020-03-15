# sleeptime_r

Sleeptime_r is a simple GTK+ application used to set a sleep timer for your PC (like the one on your television).

Thanks to the authors of the shrinkwraprs, clap, and system_shutdown crates.

It's a bit like using ```bash shutdown +m``` except the +m value is taken via GUI. At the scheduled time, a second window will appear to allow the user to opt-out of the scheduled shutdown (if you can't sleep--like me.)

## Installation

### From source:
```bash
# Clone the repository
git clone https://github.com/AKAStacks/sleeptime_r.git

# cd into the directory
cd sleeptime_r

# Use cargo to build the project
cargo build --release

# cd into the built directory
cd ./target/release

# Copy the binary to /usr/bin, or another directory in your $PATH
sudo cp sleeptime_r /usr/bin/

# Clean up
cd ../../..
rm -r ./sleeptime_r
```

### Within Arch, makepkg & pacman:
```bash
wget https://www.github.com/AKAStacks/sleeptime_r/PKGBUILD
makepkg
sudo pacman -U ./sleeptime_r-0.1.0-1-x86_64.pkg.tar.xz
```

TODO: Get the .deb hosted/available somehow.

## Usage

```bash sleeptime_r [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v               Enables verbose print statements.
    -V, --version    Prints version information

OPTIONS:
    -d, --default <INTEGER>       Sets initial value of timer (in minutes). Default: 0, Max: 480
    -s, --stillthere <INTEGER>    Sets timeout of 'still there' window (in seconds). Default: 10, Max: 255
```

Please note that 0 is a valid value for the delay. This allows for testing of the above options, or if you just want to shutdown after the default 10 seconds.

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)
