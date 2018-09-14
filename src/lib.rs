extern crate colored;

//use std::io::Write; // for stderr
use std::process::Command;
use std::thread;
use std::time;

use colored::*;

// conditional println macro
#[macro_export]
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

/* Now in Rust's standard library.
// same as print but for stderr
#[macro_export]
macro_rules! eprint {
    ( $( $arg:tt )* ) => {
        write!(&mut ::std::io::stderr(), $( $arg )*).expect("failed printing to stderr");
    };
}

// same as println but for stderr
#[macro_export]
macro_rules! eprintln {
    ( $( $arg:tt )* ) => {
        writeln!(&mut ::std::io::stderr(), $( $arg )*).expect("failed printing to stderr");
    };
}
*/

pub fn wait_and_run_once(
    verbose: bool,
    waiting_duration: time::Duration,
    full_command_str: &str,
    command: &mut Command,
    print_output: bool,
) -> Result<(), ()> {
    // === waiting ===
    cprintln!(
        verbose,
        "Next command in {} seconds.",
        waiting_duration.as_secs().to_string().green().bold()
    );
    let now = time::Instant::now();
    while waiting_duration - now.elapsed() > time::Duration::from_secs(60) {
        thread::sleep(time::Duration::from_secs(60));
        cprintln!(
            verbose,
            "{} seconds elapsed! {} seconds remaining.",
            now.elapsed().as_secs().to_string().green(),
            ((waiting_duration - now.elapsed()).as_secs() + 1)
                .to_string()
                .green()
        );
        // ^ +1 for rounding
    }
    thread::sleep(waiting_duration - now.elapsed());
    // === end waiting ===

    // === issue command ===
    let output = match command.output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!(
                "{}: {}\nReason: {}",
                "Failed to execute".red().bold(),
                full_command_str.blue().bold(),
                e
            );
            return Err(());
        }
    };
    cprintln!(
        verbose,
        "Issued command {}.",
        full_command_str.blue().bold()
    );

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
