use std::env;

use crate::error::ActionsError;

// Implemented from https://github.com/actions/toolkit/blob/main/packages/core/src/core.ts#L126
/// Obtain an input from a variable
///
/// If the input is not found, an [ActionsError] is returned
///
/// This method was copied
/// from [core.ts](https://github.com/actions/toolkit/blob/d1df13e178816d69d96bdc5c753b36a66ad03728/packages/core/src/core.ts#L126)
pub fn get_input(name:&str)-> Result<String,ActionsError> {
    let clean_input = str::replace(name, ' ', "_");
    let value = env::var(clean_input.to_uppercase());
    match value {
        Ok(input) => Ok(input),
        Err(_) => Err(ActionsError::InputNotFound(name.to_string()))
    }
}