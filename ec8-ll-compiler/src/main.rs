mod args;
mod ast;
mod parser;

use crate::args::{arg_matches, Options, read_options, setup_logging};
use crate::parser::parse;
use color_eyre::eyre::eyre;
use color_eyre::Result;
use std::fs;

fn main() -> Result<()> {
    color_eyre::install()?;

    let matches = arg_matches();

    setup_logging(&matches);
    let options = read_options(&matches)?;

    let source = fs::read_to_string(&options.input_file)?;

    let asm = process(source.lines().collect(), &options)?;

    fs::write(options.output_file, asm.join("\n"))?;

    Ok(())
}

fn process(source: Vec<&str>, options: &Options) -> Result<Vec<String>> {
    let program = parse(source).map_err(|str| eyre!("{str}"))?;

    if let Some(text) = program.warnings(options.ec8_level, options.lint_level) {
        eprintln!("Warning:\n{text}");
    }

    Ok(program.to_asm())
}

#[cfg(test)]
mod test {
    use clap_common::CheckLevel;
    use crate::args::Options;
    use crate::process;

    fn make_options() -> Options {
        Options {
            input_file: Default::default(),
            output_file: Default::default(),
            ec8_level: CheckLevel::Off,
            lint_level: CheckLevel::Off,
        }
    }

    #[test]
    fn check_progress_basic() {
        let input = vec!["lbl: v3 = xff", "data test 01a2", "goto lbl", "i = test"];
        let output = process(input, &make_options()).unwrap();
        assert_eq!(output, vec!["set v3, FF", "jmp 200", "sti 206", "dat [01A2]"].iter().map(|s| s.to_string()).collect::<Vec<String>>());
    }

    #[test]
    fn check_if() {
        let input = vec!["loop","if eq(v3,3) break", "again", "i = @0"];
        let output = process(input, &make_options()).unwrap();
        assert_eq!(output, vec!["ske v3, 03","jmp 206","jmp 200","sti 000"])
    }

    #[test]
    fn check_goto_self() {
        let input = vec!["end: goto(end)"];
        let output = process(input, &make_options()).unwrap();
        assert_eq!(output, vec!["jmp 200"])
    }
}