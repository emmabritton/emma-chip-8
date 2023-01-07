mod args;
mod parser;
mod program;

use crate::args::{get_file_names, setup_logging, LevelFilterParser};
use crate::parser::parse;
use clap::ValueHint::FilePath;
use clap::{arg, command, value_parser};
use color_eyre::Result;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = command!()
        .arg(
            arg!([INPUT_FILE] "EC8 ASM file (*.eca)")
                .required(true)
                .value_hint(FilePath)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-o --output [FILE] "Output file (defaults to input dir)")
                .value_parser(value_parser!(PathBuf))
                .value_hint(FilePath),
        )
        .arg(
            arg!(-d --desc [FILE] "Generate describe file")
                .value_parser(value_parser!(PathBuf))
                .value_hint(FilePath),
        )
        .arg(
            arg!(-l --level [LevelFilter] "Logging level")
                .value_parser(LevelFilterParser {})
                .default_value("warn"),
        )
        .get_matches();

    setup_logging(&matches);
    let options = get_file_names(&matches)?;

    let source = fs::read_to_string(options.input_file)?;
    let source = source.lines().collect();

    let program = parse(source)?;

    if let Some(desc_file) = options.desc_file {
        let result = fs::write(desc_file, program.describe());
        if let Err(err) = result {
            eprintln!("Error writing desc file: {err}");
        }
    }

    let bytes = program.into_bytes();

    fs::write(options.output_file, bytes)?;

    Ok(())
}