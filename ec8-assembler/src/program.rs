use crate::program::Line::*;
use ec8_common::OpCodes;
use ec8_common::OpCodes::SysCall;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Program {
    pub lines: Vec<Line>,
}

impl Program {
    pub fn warnings(&self, ignore_ec8: bool) -> Option<String> {
        let mut output = String::new();
        if let Some(text) = self.sys_call_warnings() {
            output.push_str(&text);
        }
        if let (false, Some(text)) = (ignore_ec8, self.ec8_warnings()) {
            if !output.is_empty() {
                output.push('\n');
            }
            output.push_str(&text);
        }
        if output.is_empty() {
            None
        } else {
            Some(output)
        }
    }

    fn ec8_warnings(&self) -> Option<String> {
        let mut ec8_calls: HashMap<OpCodes, Vec<usize>> = HashMap::new();
        for line in &self.lines {
            if let Code {
                idx,
                opcode,
                bytes: _,
                comment: _,
            } = line
            {
                if opcode.is_ec8_only() {
                    ec8_calls.entry(*opcode).or_default().push(*idx);
                }
            }
        }
        if !ec8_calls.is_empty() {
            let text = ec8_calls
                .iter()
                .map(|(op, lines)| {
                    format!(
                        "\n  {:?}: {}",
                        op,
                        lines
                            .iter()
                            .map(|num| num.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                })
                .collect::<Vec<String>>()
                .join("\n");
            Some(format!("Contains EC8 only calls:{text}"))
        } else {
            None
        }
    }

    fn sys_call_warnings(&self) -> Option<String> {
        let mut sys_calls = vec![];
        for line in &self.lines {
            if let Code {
                idx,
                opcode,
                bytes: _,
                comment: _,
            } = line
            {
                if opcode == &SysCall {
                    sys_calls.push(idx);
                }
            }
        }
        if !sys_calls.is_empty() {
            Some(format!(
                "Contains system calls (0x0nnn) on lines {}",
                sys_calls
                    .iter()
                    .map(|num| num.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
        } else {
            None
        }
    }
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
    Data {
        idx: usize,
        bytes: Vec<u8>,
        comment: String,
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

    pub fn new_data(idx: usize, bytes: Vec<u8>) -> Line {
        Data {
            idx,
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
        match self {
            Code {
                idx,
                opcode,
                bytes,
                comment: _,
            } => Code {
                idx,
                opcode,
                bytes,
                comment: text.to_string(),
            },
            Comment { .. } => self,
            Data {
                idx,
                bytes,
                comment,
            } => Data {
                idx,
                bytes,
                comment,
            },
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
            Data {
                idx: _,
                bytes,
                comment,
            } => {
                let comment = if !comment.is_empty() {
                    format!(";{}", comment)
                } else {
                    String::new()
                };
                let mut byte_str = String::new();
                for byte in bytes {
                    byte_str.push_str(&format!("{:02X}", byte));
                }
                write!(f, "DATA {byte_str}{comment}")
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
            match line {
                Code {
                    idx: _,
                    opcode: _,
                    bytes,
                    comment: _,
                } => {
                    output.extend_from_slice(&bytes);
                }
                Comment { .. } => {}
                Data {
                    idx: _,
                    bytes,
                    comment: _,
                } => {
                    output.extend_from_slice(&bytes);
                }
            }
        }
        output
    }
}
