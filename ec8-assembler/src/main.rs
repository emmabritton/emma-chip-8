mod args;
mod parser;
mod program;

use crate::args::{read_options, setup_logging, arg_matches, Options};
use crate::parser::parse;
use color_eyre::Result;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = arg_matches();

    setup_logging(&matches);
    let options = read_options(&matches)?;

    let source = fs::read_to_string(&options.input_file)?;

    let bytes = process(source.lines().collect(), &options)?;

    fs::write(options.output_file, bytes)?;

    Ok(())
}

fn process(source: Vec<&str>, options: &Options) -> Result<Vec<u8>> {
    let program = parse(source)?;

    if let Some(text) = program.warnings(options.suppress_ec8_warning) {
        eprintln!("Warning:\n{text}");
    }

    if let Some(desc_file) = &options.desc_file {
        let result = fs::write(desc_file, program.describe());
        if let Err(err) = result {
            eprintln!("Error writing desc file: {err}");
        }
    }

    Ok(program.into_bytes())
}

#[cfg(test)]
mod test {
    use crate::args::Options;
    use crate::process;

    fn make_options() -> Options {
        Options {
            input_file: Default::default(),
            output_file: Default::default(),
            desc_file: None,
            suppress_ec8_warning: false,
        }
    }

    #[test]
    fn check_process_basic() {
        let input = vec![
            "set v0, 5",
            "dat [aaaa]",
            "add v2, v1",
        ];
        let output = process(input, &make_options()).unwrap();
        assert_eq!(output, vec![0x60, 0x05, 0xAA, 0xAA, 0x82, 0x14]);
    }
}