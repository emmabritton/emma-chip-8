use crate::parser::line::tokens::Token;
use crate::parser::line::{Expected, Line};

#[derive(Clone, Debug)]
pub struct Definition {
    pub line: usize,
    pub def_type: DefinitionType,
    pub name: String,
    pub value: String,
}

impl Definition {
    pub fn new(line: usize, def_type: DefinitionType, name: String) -> Self {
        Self {
            line,
            def_type,
            name,
            value: String::new(),
        }
    }

    pub fn new_alias(line: usize, def_type: DefinitionType, name: String, value: String) -> Self {
        Self {
            line,
            def_type,
            name,
            value,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DefinitionType {
    Label,
    Alias,
    Data,
}

impl Line {
    pub fn new_error(line: usize, expected: Expected, message: String) -> Line {
        Line::Error {
            line,
            expected,
            message,
        }
    }
    pub fn new_alias(line: usize, name: String, value: String) -> Line {
        Line::Alias { line, name, value }
    }
    pub fn new_data(line: usize, name: String, data: Vec<u8>) -> Line {
        Line::Data { line, name, data }
    }

    pub fn new_code(line: usize, label: Option<String>, token: Token) -> Line {
        Line::Code { line, label, token }
    }

    pub fn new_label(line: usize, name: String) -> Line {
        Line::Label { line, name }
    }
}

impl Line {
    pub fn defs(&self) -> Vec<Definition> {
        match self {
            Line::Alias { line, name, value } => vec![Definition::new_alias(
                *line,
                DefinitionType::Alias,
                name.to_string(),
                value.to_string(),
            )],
            Line::Data {
                line,
                name,
                data: _,
            } => vec![Definition::new(
                *line,
                DefinitionType::Data,
                name.to_string(),
            )],
            Line::Code { line, label, token:_ } => {
                if let Some(lbl) = label {
                    vec![Definition::new(*line, DefinitionType::Label, lbl.to_string())]
                } else {
                    vec![]
                }
            },
            Line::Label { line, name } => vec![Definition::new(
                *line,
                DefinitionType::Label,
                name.to_string(),
            )],
            Line::Error { .. } => vec![],
        }
    }
}
