use std::ffi::OsStr;
use clap::builder::{PossibleValue, TypedValueParser};
use clap::{Arg, Command};
use clap::error::ErrorKind;
use clap::Error as ClapError;
use log::LevelFilter;

#[derive(Debug, Copy, Clone)]
pub struct LevelFilterParser {}

impl TypedValueParser for LevelFilterParser {
    type Value = LevelFilter;

    fn parse_ref(
        &self,
        _cmd: &Command,
        _arg: Option<&Arg>,
        value: &OsStr,
    ) -> Result<Self::Value, ClapError> {
        match value.to_string_lossy().to_lowercase().as_str() {
            "debug" => Ok(LevelFilter::Debug),
            "warn" => Ok(LevelFilter::Warn),
            "error" => Ok(LevelFilter::Error),
            "info" => Ok(LevelFilter::Info),
            "off" | "none" => Ok(LevelFilter::Off),
            "trace" => Ok(LevelFilter::Trace),
            _ => Err(ClapError::raw(
                ErrorKind::InvalidValue,
                "Logging level is invalid",
            )),
        }
    }

    fn possible_values(&self) -> Option<Box<dyn Iterator<Item = PossibleValue> + '_>> {
        Some(Box::new(
            vec![
                PossibleValue::new("off"),
                PossibleValue::new("error"),
                PossibleValue::new("warn"),
                PossibleValue::new("info"),
                PossibleValue::new("debug"),
                PossibleValue::new("trace"),
            ]
                .into_iter(),
        ))
    }
}

