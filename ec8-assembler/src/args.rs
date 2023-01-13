use color_eyre::Result;
use env_logger::Builder;
use log::LevelFilter;
use std::path::PathBuf;
use clap::ValueHint::FilePath;
use clap::{arg, command, value_parser, ArgMatches};
use clap_common::{arg_check_level, arg_input_file, arg_log_level, arg_output_file};
use clap_common::arg_matcher::{ArgMatchesFiles, create_output_default};
use color_eyre::eyre::eyre;

#[derive(Debug, Clone)]
pub struct Options {
    pub input_file: PathBuf,
    pub output_file: PathBuf,
    pub desc_file: Option<PathBuf>,
    pub suppress_ec8_warning: bool,
}

pub fn arg_matches() -> ArgMatches {
    command!()
        .arg(
            arg_input_file!("EC8 ASM file (*.eca)"),
        )
        .arg(
            arg_output_file!(),
        )
        .arg(
            arg!(-d --desc [FILE] "Generate describe file")
                .value_parser(value_parser!(PathBuf))
                .value_hint(FilePath),
        )
        .arg(arg_log_level!())
        .arg(arg_check_level!(ec8, e, "EC8 check level", "warn"))
        .get_matches()
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

pub fn read_options(matches: &ArgMatches) -> Result<Options> {
    let input_file = matches.get_file("INPUT_FILE", "Input file").map_err(|txt| eyre!(txt))?;
    let default_output = create_output_default(&input_file, ".c8", "Output file");
    let output = matches.get_output_file("output", "Output file", default_output).map_err(|txt| eyre!(txt))?;

    let mut desc_file = None;
    if matches.contains_id("desc") {
        let default_output = create_output_default(&input_file, ".desc", "Describe file");
        let file = matches.get_output_file("desc", "Describe file", default_output).map_err(|txt| eyre!(txt))?;
        desc_file = Some(file);
    }

    let suppress_ec8_warning = matches.contains_id("ec8");

    Ok(Options {
        input_file,
        output_file: output,
        desc_file,
        suppress_ec8_warning,
    })
}
