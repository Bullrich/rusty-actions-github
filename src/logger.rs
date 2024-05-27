//! Logging methods for the output
use std::io::Write;
use std::{env, io};

use crate::util::EOL;

fn issue_command(command: &str, msg: &str) {
    let message = format!("::{} {}", command, msg);
    io::stdout()
        .write_all((message + EOL).as_bytes())
        .expect("Failed to write message")
}

/// Prints a debug message to the log.
///
/// Only visible if [debug logging is enabled](https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/enabling-debug-logging)
///
/// ```rust
/// use actions_github::logger;
/// logger::debug_log("Initializing the project");
/// ```
pub fn debug_log(msg: &str) {
    let message = format!("::debug::{}", msg);
    io::stdout()
        .write_all((message + EOL).as_bytes())
        .expect("Failed to write debug message")
}

/// Logs regular information message
///
/// ```rust
/// use actions_github::logger;
/// logger::info("Finished analyzing project");
/// ```
pub fn info(msg: &str) {
    io::stdout()
        .write_all((msg.to_owned() + EOL).as_bytes())
        .expect("Failed to write debug message")
}

/// Creates a warning message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// use actions_github::logger;
/// logger::warn_log("Missing name of project");
/// ```
pub fn warn_log(msg: &str) {
    issue_command("warning", msg);
}

/// Creates an error message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// use actions_github::logger;
/// logger::error_log("Did not find library");
/// ```
pub fn error_log(msg: &str) {
    issue_command("error", msg);
}

/// Creates a notice message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// use actions_github::logger;
/// logger::notice_log("Step one is finished");
/// ```
pub fn notice_log(msg: &str) {
    issue_command("notice", msg);
}

/// Returns if it's running on a debug runner.
///
/// If the `RUNNER_DEBUG` variable is not defined, it'll always return true
///
/// ```rust
/// use actions_github::logger;
/// assert!(logger::is_debug());
/// ```
pub fn is_debug() -> bool {
    match env::var("RUNNER_DEBUG") {
        Ok(value) => value == "1",
        Err(_) => true,
    }
}
