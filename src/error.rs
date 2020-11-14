use std::fmt::{Display, Formatter};
use std::{error, fmt};

#[derive(Debug)]
pub enum EngineError {
    NameConflict,
}

impl Display for EngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::NameConflict => {
                write!(f, "NameConflict: A theory with this name already exists.")
            }
        }
    }
}

impl error::Error for EngineError {}
