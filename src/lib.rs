//! # The Scripting Library
//!
//! The Scripting Library provides functions to facilitate scripting
//! in Rust Llanguage.

#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;
use std::process::{Command, Output};

/// Runs a given command and return a Result holding an Output struct
///
/// TODO: examples
pub fn run_cmd(cmd: &mut Vec<String>) -> Result<Output, std::io::Error> {
    let log = slog::Logger::root(slog_term::streamer().full().build().fuse(), o!("version" => env!("CARGO_PKG_VERSION")));

    trace!(log, "Starting with Command");
    info!(log, "Starting with Command: {}", cmd.join(" "));
    let base = cmd.remove(0);
    let mut command = Command::new(base);
    for part in cmd {
        command.arg(part);
    }
    command.output()
}

#[cfg(test)]
mod tests_run_cmd {
    use super::run_cmd;

    #[test]
    fn success_output() {
        let mut cmd = vec!["echo".to_string(), "hello".to_string()];
        let exp = "hello\n";
        let res = run_cmd(&mut cmd).unwrap();
        let stdout = String::from_utf8(res.stdout).unwrap();
        assert_eq!(exp, stdout);
    }

    #[test]
    fn success_return_code() {
        let mut cmd = vec!["echo".to_string()];
        let res = run_cmd(&mut cmd).unwrap();
        assert_eq!(true, res.status.success());
    }

    #[test]
    #[should_panic(expected = "No such file or directory")]
    fn failure_cmd_absent() {
        let mut cmd = vec!["blabla".to_string()];
        let res = run_cmd(&mut cmd);
        assert_eq!(false, res.unwrap().status.success());
    }

    #[test]
    fn failure_return_code() {
        let mut cmd = vec!["cp".to_string()];
        let res = run_cmd(&mut cmd).unwrap();
        assert_eq!(false, res.status.success());
    }
}
