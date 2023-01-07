use crate::program::Line::{Code, Comment};
use ec8_common::OpCodes;
use std::fmt::{Display, Formatter};

pub struct Program {
    pub lines: Vec<Line>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Line {
    Code {
        idx: usize,
        opcode: OpCodes,
        bytes: [u8; 2],
        comment: String,
    },
    Comment {
        idx: usize,
        text: String,
    },
}

impl Line {
    pub fn new_code(idx: usize, opcode: OpCodes, bytes: [u8; 2]) -> Line {
        Code {
            idx,
            opcode,
            bytes,
            comment: String::new(),
        }
    }

    pub fn new_comment(idx: usize, comment: &str) -> Line {
        Comment {
            idx,
            text: comment.to_string(),
        }
    }

    pub fn append_comment(self, text: &str) -> Line {
        if let Code {
            idx,
            opcode,
            bytes,
            comment: _,
        } = self
        {
            Code {
                idx,
                opcode,
                bytes,
                comment: text.to_string(),
            }
        } else {
            self
        }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Code {
                idx: _,
                opcode,
                bytes,
                comment,
            } => {
                let desc = opcode.simple_describe(*bytes);
                let comment = if !comment.is_empty() {
                    format!(";{}", comment)
                } else {
                    String::new()
                };
                write!(f, "{:02X}{:02X} {} {}", bytes[0], bytes[1], desc, comment)
            }
            Comment { idx: _, text } => write!(f, ";{text}"),
        }
    }
}

impl Program {
    pub fn new(lines: Vec<Line>) -> Self {
        Self { lines }
    }
}

impl Program {
    pub fn describe(&self) -> String {
        let mut output = String::new();
        for line in &self.lines {
            output.push_str(&line.to_string());
            output.push('\n');
        }
        output
    }

    pub fn into_bytes(self) -> Vec<u8> {
        let mut output = vec![];
        for line in self.lines {
            if let Code {
                idx: _,
                opcode: _,
                bytes,
                comment: _,
            } = line
            {
                output.push(bytes[0]);
                output.push(bytes[1]);
            }
        }
        output
    }
}
