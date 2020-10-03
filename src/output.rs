//! Handlers to process output.
//!
//! It can be used to make a paging output, or just output normally(dump to stdout).
//!
//! # Examples
//!
//! Make paging output
//!
//! ```rust
//! use hors::Output;
//! use hors::config::PagingOption;
//!
//! fn run() {
//! let mut outputer = Output::new(&PagingOption::Auto);
//! let handler = outputer.get_handler();
//! handler.write_all("test data".as_bytes()).expect("success");
//! }
//! ```
//!
//! Or just want to dump to stdout.
//!
//! ```rust
//! use hors::Output;
//! use hors::config::PagingOption;
//!
//! fn run() {
//! let mut outputer = Output::new(&PagingOption::Never);
//! let handler = outputer.get_handler();
//! handler.write_all("test data".as_bytes()).expect("success");
//! }
//! ```
use crate::config::PagingOption;
use std::io::{self, Stdout, Write};
use std::process::{Child, Command, Stdio};

pub enum Output {
    /// Paging output, along with a relative output handler process.
    Paging(Child),
    /// Normal output, along with stdout.
    Normal(Stdout),
}

impl Output {
    pub fn new(option: &PagingOption) -> Output {
        // when paging option is never, we just make a normal output, in this case
        // result will output normally.
        // If we need paging, use `less` command to handle paging feature for us.
        match option {
            PagingOption::Auto => {
                // create a less process.
                Command::new("less")
                    .args(&["--raw-control-chars", "--quit-if-one-screen", "--no-init"])
                    .stdin(Stdio::piped())
                    .spawn()
                    .map_or_else(|_| Output::Normal(io::stdout()), |cmd| Output::Paging(cmd))
            }
            PagingOption::Never => Output::Normal(io::stdout()),
        }
    }

    /// Get output handler so user can write data.
    pub fn get_handler(&mut self) -> &mut dyn Write {
        match self {
            Output::Paging(child_proc) => child_proc
                .stdin
                .as_mut()
                .expect("get stdin of child process failed"),
            Output::Normal(out) => out,
        }
    }
}

// Implement this method to make relative `less` command waiting for output.
impl Drop for Output {
    fn drop(&mut self) {
        if let Output::Paging(child_proc) = self {
            let _ = child_proc.wait();
        }
    }
}
