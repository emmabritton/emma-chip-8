mod checks;
pub mod line;
mod macros;

use crate::ast::{build_opcodes, Program};
use crate::parser::checks::{handle_errors, validate_code, verify_labels};
use crate::parser::line::parse_line;
use crate::parser::macros::{expand_macros, extract_macros, Macro};
use ec8_common::MAX_PROG_SIZE;

pub fn parse(source: Vec<&str>) -> Result<Program, String> {
    let mut defs = vec![];
    let mut lines = vec![];
    for (i, line) in source.iter().enumerate() {
        if let Some(token) = parse_line(i, line, &defs) {
            defs.extend_from_slice(&token.defs());
            lines.push(token);
        }
    }

    handle_errors(&lines)?;

    validate_code(&lines)?;

    if let Some(warnings) = verify_labels(&lines)? {
        println!("Warnings: \n{warnings}");
    }

    let (lines, mut macros) = extract_macros(lines)?;

    for builtin_macro in Macro::builtin_macros() {
        macros.insert(builtin_macro.name.clone(), builtin_macro);
    }

    let lines = expand_macros(lines, macros)?;

    let mut program = build_opcodes(&lines)?;

    if (program.count_data_bytes() + program.count_asm_bytes()) > MAX_PROG_SIZE {
        return Err(format!(
            "Program and data are too large, max {MAX_PROG_SIZE}b\nProgram {}b\nData {}b",
            program.count_asm_bytes(),
            program.count_data_bytes()
        ));
    }

    program.set_addresses();

    Ok(program)
}
