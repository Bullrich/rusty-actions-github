//! Logging methods for the output
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::{env, io};

use crate::util::EOL;

const LOG_DEBUG_ENV: &str = "LOG_DEBUG";
const LOG_WRITE_ENV: &str = "LOG_DEBUG_WRITE";

#[derive(Debug)]
/// Struct which contains the parameters used for custom annotations.
///
/// This is used in the [notice_log], [warn_log] and [error_log]
///
/// ## Example use case
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger::{LogParameters, notice_log};
/// let params = LogParameters {
///     title: String::from("My example"),
///     file: String::from("src/lib.rs"),
///     line: 1,
///     end_line: 3
/// };
///
/// notice_log("There is a problem in the file", Some(params));
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(),
/// # "::notice file=src/lib.rs,line=1,endLine=3,title=My example::There is a problem in the file");
/// ```
pub struct LogParameters {
    /// Custom title
    pub title: String,
    /// Filename
    pub file: String,
    /// Line number, starting at 1
    pub line: u16,
    /// End line number
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
            title: String::from(""),
            file: String::from(".github"),
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
/// GitHub's documentation: [Setting a debug message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-debug-message)
///
/// ## Example usage
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
/// ## Example usage
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
/// GitHub's documentation: [Setting a warning message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-warning-message)
///
/// ## Example usage
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::warn_log("Missing name of project", None);
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::warning::Missing name of project")
/// ```
///
/// ## Custom annotations
///
/// You can also set custom annotations to mention a specific line in a file.
/// See [LogParameters] for more info.
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::warn_log("Missing name of project", Some(logger::LogParameters {
///     title: String::from("Missing name"),
///     file: String::from("src/lib.rs"),
///     line: 1,
///     end_line: 3
/// }));
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(),
/// #   "::warning file=src/lib.rs,line=1,endLine=3,title=Missing name::Missing name of project")
/// ```
pub fn warn_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("warning", msg, parameters);
}

/// Creates an error message and prints the message to the log.
///
/// This message will create an annotation.
///
/// GitHub's documentation: [Setting an error message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-error-message)
///
/// ## Example usage
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::error_log("Did not find library", None);
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::error::Did not find library");
/// ```
///
/// ### Custom annotations
///
/// You can also set custom annotations to mention a specific line in a file.
/// See [LogParameters] for more info.
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::error_log("Did not find library", Some(logger::LogParameters {
///     title: String::from("Library missing"),
///     file: String::from("Cargo.toml"),
///     line: 4,
///     end_line: 7
/// }));
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(),
/// #   "::error file=Cargo.toml,line=4,endLine=7,title=Library missing::Did not find library")
/// ```
pub fn error_log(msg: &str, parameters: Option<LogParameters>) {
    issue_command("error", msg, parameters);
}

/// Creates a notice message and prints the message to the log.
///
/// This message will create an annotation.
///
/// GitHub's Documentation: [Setting a notice message](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-a-notice-message)
///
/// ## Example usage
///
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::notice_log("Step one is finished", None);
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(), "::notice::Step one is finished")
/// ```
///
/// ## Custom annotations
///
/// You can also set custom annotations to mention a specific line in a file.
/// See [LogParameters] for more info.
/// ```rust
/// # std::env::set_var("LOG_DEBUG", "true");
/// use actions_github::logger;
/// logger::notice_log("Step one is finished", Some(logger::LogParameters {
///     title: String::from("Step completed"),
///     file: String::from(".github/workflows/test.yml"),
///     line: 24,
///     end_line: 27
/// }));
/// # assert_eq!(std::env::var("LOG_DEBUG_WRITE").unwrap(),
/// #   "::notice file=.github/workflows/test.yml,line=24,endLine=27,title=Step completed::Step one is finished")
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
