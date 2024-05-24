use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
#[non_exhaustive]
/// Error occurred duration the execution of the action
pub enum ActionsError {
    /// This error happened while generating the context object
    Context(String),
    InputNotFound(String)
}

impl Error for ActionsError {}

impl Display for ActionsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ActionsError::*;

        match self {
            Context(msg) => write!(f, "Problem while generating the context: {}", msg),
        }
    }
}
