use clap::builder::{PossibleValue, TypedValueParser};
use clap::error::{Error as ClapError, ErrorKind};
use clap::{Arg, ArgMatches, Command};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use env_logger::Builder;
use log::LevelFilter;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Options {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub desc_file: Option<PathBuf>,
}

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

pub fn setup_logging(matches: &ArgMatches) {
    Builder::from_default_env()
        .format_module_path(false)
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .filter(
            Some("ec8_assembler"),
            *matches
                .get_one::<LevelFilter>("level")
                .expect("Invalid level filter"),
        )
        .init();
}

pub fn get_file_names(matches: &ArgMatches) -> Result<Options> {
    let input_file = matches
        .get_one::<PathBuf>("INPUT_FILE")
        .cloned()
        .expect("Input file must be provided");
    if !input_file.is_file() {
        return Err(eyre!(format!(
            "Input file {} is not a file",
            input_file.display()
        )));
    }
    let filename = input_file.file_stem().unwrap_or_else(|| {
        panic!(
            "Input file {} has invalid path/filename",
            input_file.display()
        )
    });
    let path = input_file.parent().unwrap_or_else(|| {
        panic!(
            "Input file {} has invalid path/location",
            input_file.display()
        )
    });
    let mut default_output = PathBuf::from(path);
    default_output.push(filename);
    if !default_output.set_extension(".c8") {
        panic!("Unable to set output extension, please raise a bug");
    }
    let output = matches
        .get_one::<PathBuf>("output")
        .cloned()
        .unwrap_or(default_output);

    let mut desc_file = None;
    if matches.contains_id("desc") {
        let file = matches
            .get_one::<PathBuf>("desc")
            .cloned()
            .expect("Describe file path is invalid");
        desc_file = Some(file);
    }

    Ok(Options {
        input_file,
        output_file: output,
        desc_file,
    })
}
