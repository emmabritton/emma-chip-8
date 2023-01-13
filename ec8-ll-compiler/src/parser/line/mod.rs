use crate::parser::line::models::Definition;
use crate::parser::line::parsers::{parse_alias, parse_code, parse_data, parse_label};
use crate::parser::line::tokens::Token;

pub mod models;
pub mod parsers;
pub mod tokens;
pub mod utils;

#[derive(Clone, Debug, PartialEq)]
pub enum Line {
    Alias {
        line: usize,
        name: String,
        value: String,
    },
    Data {
        line: usize,
        name: String,
        data: Vec<u8>,
    },
    Code {
        line: usize,
        label: Option<String>,
        token: Token,
    },
    Label {
        line: usize,
        name: String,
    },
    Error {
        line: usize,
        expected: Expected,
        message: String,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Expected {
    Alias,
    Data,
    Code,
    Label,
}

pub fn parse_line(i: usize, line: &str, defs: &[Definition]) -> Option<Line> {
    let (code, _) = line.split_once(';').unwrap_or((line, ""));
    let line = code.trim();

    if line.is_empty() {
        return None;
    }

    if line.starts_with("alias") {
        parse_alias(i, line, defs)
    } else if line.starts_with("data") {
        parse_data(i, line, defs)
    } else if line.ends_with(':') {
        parse_label(i, line, defs)
    } else {
        parse_code(i, line, defs)
    }
}
