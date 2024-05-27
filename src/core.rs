//! Utility methods to interact with the GitHub actions ecosystem
//!
//! You can obtain injected inputs or produce an output for another step
use std::io::Write;
use std::{env, io};

use crate::error::ActionsError;
use crate::util::{issue_file_command, issue_old_command, prepare_key_value_message, EOL};

/// Obtain an input from a variable
///
/// If the input is not found, an [ActionsError] is returned
///
/// This method was copied
/// from [core.ts](https://github.com/actions/toolkit/blob/d1df13e178816d69d96bdc5c753b36a66ad03728/packages/core/src/core.ts#L126)
///
/// ```rust
/// use actions_github::logger;
/// use actions_github::core;
///
/// let name = core::get_input("name").unwrap_or_else(|_| {
///     logger::warn_log("Input 'name' was not defined");
///     String::from("")
/// });
/// ```
pub fn get_input(name: &str) -> Result<String, ActionsError> {
    let mut clean_input = str::replace(name, ' ', "_");
    clean_input.insert_str(0, "INPUT_");
    let value = env::var(clean_input.to_uppercase());
    match value {
        Ok(input) => Ok(input),
        Err(_) => Err(ActionsError::InputNotFound(name.to_string())),
    }
}

/// Produces an [output](https://docs.github.com/en/actions/using-workflows/workflow-commands-for-github-actions#setting-an-output-parameter)
/// that can be used in another step
///
/// ```rust
/// use actions_github::core;
/// if let Err(err) = core::set_output("name", "value") {
///     panic!("{:#?}", err);
/// }
/// ```
pub fn set_output(name: &str, value: &str) -> Result<(), ActionsError> {
    if env::var("GITHUB_OUTPUT").is_ok() {
        return match prepare_key_value_message(name, value) {
            Ok(key_value_message) => match issue_file_command("OUTPUT", key_value_message) {
                Ok(_) => Ok(()),
                Err(err) => Err(ActionsError::Output(err)),
            },
            Err(err) => Err(ActionsError::Output(err)),
        };
    }

    io::stdout()
        .write_all(EOL.as_bytes())
        .expect("Failed to write EOL");
    issue_old_command("set-output", name, value);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::core::{get_input, set_output};
    use std::env;

    #[test]
    fn returns_input_when_env_is_set() {
        env::set_var("INPUT_EXAMPLE", "test");
        let input = get_input("example");
        assert_eq!(input.unwrap(), "test")
    }

    #[test]
    fn returns_error_when_env_is_not_set() {
        let input = get_input("test");
        assert!(input.is_err())
    }

    #[test]
    fn writes_output() {
        assert!(set_output("hi", "bye").is_ok());
    }
}
