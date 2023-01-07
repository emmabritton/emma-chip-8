use std::error::Error;
use std::fmt::{Display, Formatter};
use ECoreError::*;

pub type ECoreResult<T> = Result<T, ECoreError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ECoreError {
    ProgramTooLarge,
}

impl Display for ECoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramTooLarge => write!(f, "Program is too large"),
        }
    }
}

impl Error for ECoreError {}
