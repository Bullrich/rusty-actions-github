use std::env;

use crate::error::ActionsError;

// Implemented from https://github.com/actions/toolkit/blob/main/packages/core/src/core.ts#L126
/// Obtain an input from a variable
///
/// If the input is not found, an [ActionsError] is returned
///
/// This method was copied
/// from [core.ts](https://github.com/actions/toolkit/blob/d1df13e178816d69d96bdc5c753b36a66ad03728/packages/core/src/core.ts#L126)
pub fn get_input(name: &str) -> Result<String, ActionsError> {
    let mut clean_input = str::replace(name, ' ', "_");
    clean_input.insert_str(0, "INPUT_");
    let value = env::var(clean_input.to_uppercase());
    match value {
        Ok(input) => Ok(input),
        Err(_) => Err(ActionsError::InputNotFound(name.to_string())),
    }
}

#[cfg(test)]
mod test {
    use crate::core::get_input;
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
}
