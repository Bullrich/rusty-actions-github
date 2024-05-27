//! Logging methods for the output
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::{env, io};

use crate::util::EOL;

const LOG_DEBUG_ENV: &str = "LOG_DEBUG";
const LOG_WRITE_ENV: &str = "LOG_DEBUG_WRITE";

#[derive(Debug)]
pub struct LogParameters {
    pub title: String,
    pub file: String,
    pub line: u16,
    pub end_line: u16,
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
    match env::var(LOG_DEBUG_ENV) {
        Ok(_) => env::set_var(LOG_WRITE_ENV, message),
        Err(_) => io::stdout()
            .write_all((message + EOL).as_bytes())
            .expect("Failed to write message"),
    }
}

/// Prints a debug message to the log.
///
/// Only visible if [debug logging is enabled](https://docs.github.com/en/actions/monitoring-and-troubleshooting-workflows/enabling-debug-logging)
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::debug_log("Initializing the project");
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::debug::Initializing the project")
/// ```
pub fn debug_log(msg: &str) {
    let message = format!("::debug::{}", msg);

    match env::var(LOG_DEBUG_ENV) {
        Ok(_) => env::set_var(LOG_WRITE_ENV, message),
        Err(_) => io::stdout()
            .write_all((message + EOL).as_bytes())
            .expect("Failed to write debug message"),
    }
}

/// Logs regular information message
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::info(format!("Finished analyzing {}", "project").as_str());
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "Finished analyzing project")
/// ```
pub fn info(msg: &str) {
    match env::var(LOG_DEBUG_ENV) {
        Ok(_) => env::set_var(LOG_WRITE_ENV, msg),
        Err(_) => io::stdout()
            .write_all((msg.to_owned() + EOL).as_bytes())
            .expect("Failed to write info message"),
    }
}

/// Creates a warning message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::warn_log("Missing name of project", None);
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::warning::Missing name of project")
/// ```
pub fn warn_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("warning", msg, parameters);
}

/// Creates an error message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::error_log("Did not find library", None);
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::error::Did not find library");
/// ```
pub fn error_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("error", msg, parameters);
}

/// Creates a notice message and prints the message to the log.
///
/// This message will create an annotation.
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::notice_log("Step one is finished", None);
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::notice::Step one is finished")
/// ```
pub fn notice_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("notice", msg, parameters);
}

/// Returns if it's running on a debug runner.
///
/// If the `RUNNER_DEBUG` variable is not defined, it'll always return true
///
/// ```rust
/// use actions_github::logger::is_debug;
/// assert!(is_debug());
/// std::env::set_var("RUNNER_DEBUG", "0");
/// assert!(!is_debug());
/// ```
pub fn is_debug() -> bool {
    match env::var("RUNNER_DEBUG") {
        Ok(value) => value == "1",
        Err(_) => true,
    }
}
