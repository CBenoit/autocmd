///////////////////////////////////////////////////////////////////////
// Copyright (c) 2017 Benoît C. <benoit.cortier@fried-world.eu>      //
// Licensed under the MIT license <LICENSE.txt or                    //
// http://opensource.org/licenses/MIT>. This file may not be copied, //
// modified, or distributed except according to those terms.         //
///////////////////////////////////////////////////////////////////////

#[macro_use]
extern crate clap;
extern crate colored;

use std::process::Command;
use std::thread;
use std::time;
use std::io::Write; // for stderr

use colored::*;

// conditional println macro
macro_rules! cprintln {
    ( $cond:ident ) => {
        if $cond {
            println!();
        }
    };
    ( $cond:ident, $( $x:expr ),* ) => {
        if $cond {
            println!($( $x ),*);
        }
    };
}

// same as print but for stderr
macro_rules! eprint {
    ( $( $arg:tt )* ) => {
        write!(&mut ::std::io::stderr(), $( $arg )*).expect("failed printing to stderr");
    };
}

// same as println but for stderr
macro_rules! eprintln {
    ( $( $arg:tt )* ) => {
        writeln!(&mut ::std::io::stderr(), $( $arg )*).expect("failed printing to stderr");
    };
}

fn is_an_unsigned_integer(val: String) -> Result<(), String> {
    if val.parse::<u64>().is_ok() {
        Ok(())
    } else {
        Err(String::from("the value must be a positive integer."))
    }
}

fn main() {
    let matches = clap_app!(AutoCMD =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "A simple program that issues a command a certain amount of times with a given interval.")
        (after_help: "If repeat option is not provided the program shall run indefinitely.")
        (@arg command: +required +last +multiple "The command to issue")
        (@arg interval: +required +takes_value {is_an_unsigned_integer} -i --interval "Interval between commands in seconds")
        (@arg repeats: +takes_value {is_an_unsigned_integer} -r --repeat "Repeat <repeats> times and stop")
        (@arg quiet: -q --quiet "Disable AutoCMD standard outputs (doesn't cancel --print_output)")
        (@arg print_output: -o --print_output "Show the chosen command outputs in the standard output (dosen't cancel --quiet)")
    ).get_matches();

    let verbose = !matches.is_present("quiet");
    let print_output = matches.is_present("print_output");

    // here the unwraps are safe since "interval" is required and checked by clap.
    let interval_secs = matches.value_of("interval").unwrap().parse::<u64>().unwrap();

    // safe since "command" is required by clap
    let full_command_str = get_full_command_str_from_values(matches.values_of("command").unwrap());
    let mut command_and_args_iter = matches.values_of("command").unwrap();
    let mut command = Command::new(command_and_args_iter.next().unwrap());
    for arg in command_and_args_iter {
        command.arg(arg);
    }

    cprintln!(verbose, "{} {}!\n", "AutoCMD".yellow().bold().italic(), "started".green().bold());

    let waiting_duration = time::Duration::from_secs(interval_secs);
    match matches.value_of("repeats") {
        None => loop {
            match wait_and_run_once(verbose, waiting_duration, full_command_str.as_str(), &mut command, print_output) {
                Ok(_) => (),
                Err(_) => break
            }
        },
        Some(repeat_input) => {
            // here the unwrap is safe since "repeat" is checked by clap.
            let number_of_repeats = repeat_input.parse::<u64>().unwrap();
            for remaining_repeats in (0..number_of_repeats).rev() {
                match wait_and_run_once(verbose, waiting_duration, full_command_str.as_str(), &mut command, print_output) {
                    Ok(_) => (),
                    Err(_) => break
                }

                if remaining_repeats > 0 {
                    cprintln!(verbose, "{} repeats remaining.", remaining_repeats.to_string().yellow().bold());
                }
            }
        }
    }
}

fn get_full_command_str_from_values(values: clap::Values) -> String {
    let mut vals_vec = Vec::new();
    for val in values {
        vals_vec.push(val);
    }
    vals_vec.join(" ")
}

fn wait_and_run_once(verbose: bool, waiting_duration: time::Duration,
                     full_command_str: &str, command: &mut Command,
                     print_output: bool) -> Result<(), ()> {
    // === waiting ===
    cprintln!(verbose, "Next command in {} seconds.", waiting_duration.as_secs().to_string().green().bold());
    let now = time::Instant::now();
    while waiting_duration - now.elapsed() > time::Duration::from_secs(60) {
        thread::sleep(time::Duration::from_secs(60));
        cprintln!(verbose, "{} seconds elapsed! {} seconds remaining.",
                  now.elapsed().as_secs().to_string().green(),
                  ((waiting_duration - now.elapsed()).as_secs() + 1).to_string().green());
                                                              // ^ +1 for rounding
    }
    thread::sleep(waiting_duration - now.elapsed());
    // === waiting finished ===

    // === issue command ===
    let output = match command.output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("{}: {}\nReason: {}", "Failed to execute".red().bold(), full_command_str.blue().bold(), e);
            return Err(());
        }
    };
    cprintln!(verbose, "Issued command {}.", full_command_str.blue().bold());

    if print_output {
        if output.status.success() {
            cprintln!(verbose, "Command {}:", "succeeded".green().bold());
        } else {
            cprintln!(verbose, "Command {}:", "failed".red().bold());
        }

        print!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));
    } else {
        cprintln!(verbose);
    }
    // === command issued ===

    Ok(())
}

