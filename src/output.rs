use crate::config::PagingOption;
use std::io::{self, Stdout, Write};
use std::process::{Child, Command, Stdio};

pub enum Output {
    Paging(Child),
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
                    .args(&[
                        "--raw-control-chars",
                        "--quit-if-one-screen",
                        "--no-init",
                        "-N",
                    ])
                    .stdin(Stdio::piped())
                    .spawn()
                    .map_or_else(|_| Output::Normal(io::stdout()), |cmd| Output::Paging(cmd))
            }
            PagingOption::Never => Output::Normal(io::stdout()),
        }
    }

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
