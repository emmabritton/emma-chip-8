use std::ffi::OsStr;
use clap::builder::{PossibleValue, TypedValueParser};
use clap::{Arg, Command, Error};
use clap::error::ErrorKind;
use clap::Error as ClapError;
use crate::CheckLevel;

#[derive(Debug, Copy, Clone)]
pub struct CheckLevelParser {}

impl TypedValueParser for CheckLevelParser {
    type Value = CheckLevel;

    fn parse_ref(&self, _cmd: &Command, _arg: Option<&Arg>, value: &OsStr) -> Result<Self::Value, Error> {
        match value.to_string_lossy().to_lowercase().as_str() {
            "warn" => Ok(CheckLevel::Warn),
            "error" => Ok(CheckLevel::Error),
            "off" | "none" => Ok(CheckLevel::Off),
            _ => Err(ClapError::raw(
                ErrorKind::InvalidValue,
                "Check level is invalid",
            )),
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item=PossibleValue> + '_>> {
        Some(Box::new(
            vec![
                PossibleValue::new("off"),
                PossibleValue::new("warn"),
                PossibleValue::new("error"),
            ]
                .into_iter(),
        ))
    }
}