use std::env;

use crate::error::ActionsError;
pub fn get_input(name:&str)-> Result<String,ActionsError> {    let clean_input = str::replace(name, ' ', "_");    let value = env::var(clean_input.to_uppercase());    match value {        Ok(input) => Ok(input),        Err(_) => Err(ActionsError::InputNotFound(name.to_string()))    }}