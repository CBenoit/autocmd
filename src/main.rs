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

fn main() {
    let matches = clap_app!(AutoCMD =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: "A simple program that issues a command a certain amount of times with a given interval.")
        (after_help: "If repeat option is not provided the program shall run indefinitely.")
        (@arg command: +required "The command to issue")
        (@arg interval: +required +takes_value -i --interval "Interval between commands in seconds")
        (@arg repeat: +takes_value -r --repeat "Repeat <repeat> times and stop")
        (@arg quiet: -q --quiet "Disable AutoCMD standard outputs (doesn't cancel --print_output)")
        (@arg print_output: -o --print_output "Show the chosen command outputs in the standard output (dosen't cancel --quiet)")
    ).get_matches();

    let verbose = !matches.is_present("quiet");
    let print_output = matches.is_present("print_output");

    // first unwrap is safe since interval is required
    let interval_secs = matches.value_of("interval").unwrap().parse::<u64>().unwrap_or_else(|e| {
        panic!("unable to parse interval as unsigned integer: {}", e)
    });

    let mut remaining_repeats = matches.value_of("repeat").unwrap_or_else(|| {
        "-1"
    }).parse::<i64>().unwrap_or_else(|e| {
        panic!("unable to parse number of repeats as integer: {}", e)
    });

    let full_command = matches.value_of("command").unwrap(); // safe since command is required
    let mut command_iter = full_command.split_whitespace();
    let command_name = command_iter.next().unwrap(); // safe since there is at least one element
    let mut command = Command::new(command_name);
    for arg in command_iter {
        command.arg(arg);
    }

    cprintln!(verbose, "{} {}!\n", "AutoCMD".yellow().bold().italic(), "started".green().bold());

    let waiting_duration = time::Duration::from_secs(interval_secs);
    while remaining_repeats != 0 {
        // === waiting ===
        cprintln!(verbose, "Next command in {} seconds.", interval_secs.to_string().green().bold());
        let now = time::Instant::now();
        while waiting_duration - now.elapsed() > time::Duration::from_secs(60) {
            thread::sleep(time::Duration::from_secs(60));
            cprintln!(verbose, "{} seconds elapsed! {} seconds remaining.",
                      now.elapsed().as_secs().to_string().green(),
                      ((waiting_duration - now.elapsed()).as_secs() + 1).to_string().green());
            //                                                     ^ +1 for rounding
        }
        thread::sleep(waiting_duration - now.elapsed());
        // === waiting finished ===

        // === issue command ===
        let output = command.output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });
        cprintln!(verbose, "Issued command {}.", full_command.blue().bold());

        if print_output {
            if output.status.success() {
                let s = String::from_utf8_lossy(&output.stdout);

                cprintln!(verbose, "Command {}, stdout:", "succeeded".green().bold());
                println!("{}", s);
            } else {
                let s = String::from_utf8_lossy(&output.stderr);

                cprintln!(verbose, "Command {}, stderr:", "failed".red().bold());
                eprintln!("{}", s);
            }
        } else {
            cprintln!(verbose);
        }
        // === command issued ===

        if remaining_repeats > 0 {
            remaining_repeats -= 1;
            cprintln!(verbose, "{} repeats remaining.", remaining_repeats.to_string().yellow().bold());
        }
    }
}

