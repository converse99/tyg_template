//! When starting a Rust command line project there are certain key features that are often required
//! in all such projects:
//!
//! - The ability to parse the command line
//! - Sensible and informative error trapping
//!
//! The `tyg_template` project aims to provide both of these features enabling a consistent and
//! simple start to any command line coding project. Rather than creating a new command line parser
//! I've elected to use the excellent ['clap'](https://docs.rs/clap/latest/clap) crate.
//!
//! In case you're wondering, 'tyg' stands for 'there you go'.
//!
//! # Examples
//!
//! *In the following examples, a Unix like operating system is assumed.*
//!
//! The best way to use this is simply to do a git clone and then rename everything to suit your
//! own coding project. I would not recommend using it in its raw form directly, however, before
//! modifying the template to suit your own application, you could try running the sample
//! application by entering the following at the command prompt:
//!
//! ```text
//! $ cargo run -- --help
//! ```
//!
//! This will display the help option.
//!
//! You could also try the following:
//!
//! ```text
//! $ cargo run -- fail
//! tyg_template: src/lib.rs:122:9: Error thrown to demonstrate the error handling process
//! ```
//! 
//! This is what I call a disclosed error showing the name of the source file and where in the
//! source file the error occured.
//!
//! It is also possible to throw non-disclosed errors in which the origin is not disclosed to the
//! end user, however, this can be overridden by compiling with the `disclose` feature enabled.
//! Non-disclosed errors are useful to provide feedback to the end user in which the source
//! location is not disclosed to the user.
//!
//! ```text
//! $ cargo run -- fail --bare
//! tyg_template: Error thrown to demonstrate the error handling process
//! ```
//!
//! The same again but this time compiled with the `disclose` feature enabled.
//!
//! ```text
//! $ cargo run --features=disclose -- fail --bare
//! tyg_template: src/lib.rs:132:9: Error thrown to demonstrate the error handling process
//! ```
//!
//! Notice that the error message now shows the location of the error.
//!
//! In general, disclosed errors are ideal for debugging purposes, so during a debug session I
//! would recommend compiling the application with the `disclose` feature enabled.
//!
//! # Usage
//!
//! As mentioned previously, the best way to use this is simply to do a git clone and then rename
//! everything to suit your own coding project. My recommendation is that the cli and run functions
//! in lib.rs are moved to the main program (main.rs). This enables you to start with a clean new
//! lib.rs. Using this configuration confines the command line interface and top level error
//! trapping to main.rs. The library crate will then be responsible for the overall functionality
//! of the application.
//!
//! # Initial File Structure
//!
//! ```text
//!  tyg_template --|
//!                 |- Cargo.toml
//!                 |- Cargo.lock
//!                 |- LICENCE
//!                 |- README.md
//!                 |- src
//!                     |- main.rs {The main program}
//!                     |- lib.rs {The library crate root}
//!                     |- error.rs {The error handler}
//! ```

mod error;
pub use error::{Error, Result};

use std::ffi::OsStr;
use std::fs::File;

use clap::{arg, Command};

// This should be compiled using Cargo so that the verson number can be extracted
const VERSION: &str = env!("CARGO_PKG_VERSION");

// The cli function is used to specify the form of the command line using the builder style. You
// will need to modify this to suit your own application.
fn cli() -> Command<'static> {
    Command::new("tyg_template")
        .version(VERSION)
        .about("A demonstration of a basic command line application using clap with error handling. \
               This is designed to be used as a basic template when starting a new command line project")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(arg!(-d --debug "Show debugging information. Not currently used")
             .global(true))
        .subcommand(
            Command::new("fail")
            .about("Show how to return an error using the error handler")
            .arg(arg!(--bare "Show error without source file and line number displayed")))
        .subcommand(
            Command::new("recursive_fail")
            .about("Show how to handle errors whilst extracting values from an iterator"))
        .subcommand(
            Command::new("file_fail")
            .about("Show how to handle a regular filing system error e.g. file not found")
            .arg(arg!(--better "A better rendition of the error message"))
            .arg(arg!(<PATH> "Path to an invalid file (i.e. one that doesn't exist)").allow_invalid_utf8(true)))
}

/// Process the command line using clap
///
/// # Example
/// ```no_run
/// let answer = tyg_template::run();
///
/// println!("{:?}", answer);
/// ```
pub fn run() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("fail", sub_matches)) => {
            let bare = sub_matches.is_present("bare");
            let _ = error_demo(bare)?;
            println!("This should not be displayed because an error was forced...");
        }
        Some(("recursive_fail", _sub_matches)) => {
            let _ = recursive_fail_demo()?;
            println!("This should not be displayed because an error was forced...");
        }
        Some(("file_fail", sub_matches)) => {
            let better = sub_matches.is_present("better");
            let path = sub_matches.value_of_os("PATH").ok_or_else(|| option_err!("No path specified"))?;
            let _ = file_fail_demo(better, path)?;
            println!("Now see what happens when an invalid file is entered");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
    Ok(())
}

/// Generate a custom error
///
/// # Examples
/// ```
/// use tyg_template;
///
/// // Show error message and its location
/// let answer = tyg_template::error_demo(false);
///
/// assert!(answer.is_err());
/// println!("{:?}", answer);
///
/// // Show error message without the source location, however if the code is compiled
/// // with the 'diagnose' feature it will still display the source of the error.
/// // Use the bare error for end user facing error messages
/// let answer = tyg_template::error_demo(true);
///
/// assert!(answer.is_err());
/// println!("{:?}", answer);
/// ```
pub fn error_demo(bare: bool) -> Result<()> {
    if bare {
        result_err_bare!("Error thrown to demonstrate the error handling process")
    } else {
        result_err!("Error thrown to demonstrate the error handling process")
    }
}

// Define a counter for an iterator instance
struct Counter {
    count: u32,
}

impl Counter {
    // Initialize the counter
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // The Item type should be a Result so that we can trap
    // any errors that occur
    type Item = Result<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 11 {
            self.count += 1;
            if self.count >= 5 {
                // Flag an error situation
                Some(result_err!("Failed at cycle {}", self.count))
            } else {
                Some(Ok(self.count))
            }
        } else {
            None
        }
    }
}

/// Generate a custom error from an iterator
///
/// # Examples
/// ```
/// use tyg_template;
///
/// // Show error message and its location
/// let answer = tyg_template::recursive_fail_demo();
///
/// assert!(answer.is_err());
/// println!("{:?}", answer);
/// ```
pub fn recursive_fail_demo() -> Result<()> {
    println!("We need to fail at cycle 5");
    let counter = Counter::new();
    for n in counter {
        // The ? will trap the error and return it to the calling context
        // If the n value was ok then extract it and then print to screen
        println!("Cycle {}", n?);
    }
    Ok(())
}

/// Generate a standard error message or a custom one when attempting to open a file
///
/// # Examples
/// ```
/// use std::ffi::OsString;
/// use tyg_template;
///
/// let file_name = OsString::from("Non-existent file name"); 
///
/// // Show error message generated by std::io
/// let answer = tyg_template::file_fail_demo(false, &file_name);
///
/// assert!(answer.is_err());
/// println!("{:?}", answer);
///
/// // Show custom error message
/// let answer = tyg_template::file_fail_demo(true, &file_name);
///
/// assert!(answer.is_err());
/// println!("{:?}", answer);
/// ```
pub fn file_fail_demo(better: bool, path: &OsStr) -> Result<()> {
    let file = File::open(path);
    if better {
        // do something a bit better
        file.or_else(|e| result_err!("{}: {}", path.to_string_lossy(), e))?;
    } else {
        file?;
    }
    Ok(())
}
