//! Logging methods for the output
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::{env, io};

use crate::util::EOL;

#[derive(Debug)]
pub struct LogParameters {
    title: String,
    file: String,
    line: u16,
    end_line: u16,
}

impl Display for LogParameters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "file={},line={},endLine={},title={}",
            self.file, self.line, self.end_line, self.title
        )
    }
}

impl Default for LogParameters {
    fn default() -> Self {
        LogParameters {
            title: "".to_string(),
            file: ".github".to_string(),
            line: 1,
            end_line: 1,
        }
    }
}

fn issue_command(command: &str, msg: &str, parameters: Option<LogParameters>) {
    let message = match parameters {
        None => format!("::{}::{}", command, msg),
        Some(params) => format!("::{} {}::{}", command, params, msg),
    };
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
/// logger::info(format!("Finished analyzing {}", "project").as_str());
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
/// logger::warn_log("Missing name of project", Option::None);
/// ```
pub fn warn_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("warning", msg, parameters);
}

/// Creates an error message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// use actions_github::logger;
/// logger::error_log("Did not find library", Option::None);
/// ```
pub fn error_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("error", msg, parameters);
}

/// Creates a notice message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// use actions_github::logger;
/// logger::notice_log("Step one is finished", Option::None);
/// ```
pub fn notice_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("notice", msg, parameters);
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
