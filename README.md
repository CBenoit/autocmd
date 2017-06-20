# AutoCMD

A simple program that issues a command a certain amount of times with a given interval.

![Example screenshot](http://i.imgur.com/5KYLaXs.png)

## Usage

```
autocmd [FLAGS] [OPTIONS] --interval <interval> [--] <command>...
```
With <command> the command to issue.

Available flags:

    -h, --help            Prints help information
    -o, --print_output    Show the chosen command outputs in the standard output (dosen't cancel --quiet)
    -q, --quiet           Disable AutoCMD standard outputs (doesn't cancel --print_output)
    -V, --version         Prints version information

Available options:

    -i, --interval <interval>    Interval between commands in seconds
    -r, --repeat <repeats>       Repeat <repeats> times and stop

Note that if repeat option is not provided the program shall run indefinitely.

I personally sometimes use it like so:
```
$ autocmd -i 300 -- cvlc --play-and-stop --play-and-exit --quiet notif.wav
```
To indefinitely play a sound every 5 minutes with `cvlc`.

## Build

This program is written in [Rust](https://www.rust-lang.org). As such, you can simply build it with cargo:
```
$ cargo build --release
```
The `autocmd` program shall be found in the folder `./target/release`.

