use clap::{command, ArgMatches};
use clap_common::arg_matcher::{create_output_default, ArgMatchesFiles};
use clap_common::{arg_check_level, arg_input_file, arg_log_level, arg_output_file, CheckLevel};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use env_logger::Builder;
use log::LevelFilter;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Options {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub ec8_level: CheckLevel,
    pub lint_level: CheckLevel,
}

pub fn arg_matches() -> ArgMatches {
    command!()
        .arg(arg_input_file!("EC8 code file (*.ecc)"))
        .arg(arg_output_file!())
        .arg(arg_log_level!())
        .arg(arg_check_level!(ec8, e, "EC8 check level", "warn"))
        .arg(arg_check_level!(
            warnings,
            w,
            "Lint/warnings check level",
            "warn"
        ))
        .get_matches()
}

pub fn setup_logging(matches: &ArgMatches) {
    Builder::from_default_env()
        .format_module_path(false)
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .filter(
            Some("ec8_ll_compiler"),
            *matches
                .get_one::<LevelFilter>("level")
                .expect("Invalid level filter"),
        )
        .init();
}

pub fn read_options(matches: &ArgMatches) -> Result<Options> {
    let input_file = matches
        .get_file("INPUT_FILE", "Input file")
        .map_err(|txt| eyre!(txt))?;
    let default_output = create_output_default(&input_file, ".eca", "Output file");
    let output = matches
        .get_output_file("output", "Output file", default_output)
        .map_err(|txt| eyre!(txt))?;

    let ec8_level = *matches
        .get_one::<CheckLevel>("ec8")
        .expect("Invalid EC8 arg");
    let lint_level = *matches
        .get_one::<CheckLevel>("warnings")
        .expect("Invalid Lint arg");

    Ok(Options {
        input_file,
        output_file: output,
        ec8_level,
        lint_level,
    })
}
