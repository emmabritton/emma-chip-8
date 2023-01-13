pub mod addresses;
pub mod asm;
pub mod checks;
mod data;

use crate::ast::data::{extract_data, Data};
use crate::parser::line::tokens::Param::*;
use crate::parser::line::tokens::{Condition, Param, Token};
use crate::parser::line::Line;
use ec8_common::OpCodes;
use ec8_common::OpCodes::*;

#[derive(Debug, Clone)]
pub struct Program {
    pub datas: Vec<Data>,
    pub asm_lines: Vec<AsmLine>,
}

impl Program {
    pub fn count_data_bytes(&self) -> usize {
        self.datas.iter().map(|data| data.bytes.len()).sum()
    }

    pub fn count_asm_bytes(&self) -> usize {
        self.asm_lines.len() * 2
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmLine {
    pub line: usize,
    pub labels: Vec<String>,
    pub opcode: OpCodes,
    pub params: Vec<Param>,
}

impl AsmLine {
    pub fn new(line: usize, labels: Vec<String>, opcode: OpCodes, params: Vec<Param>) -> Self {
        Self {
            line,
            labels,
            opcode,
            params,
        }
    }
}

pub fn build_opcodes(lines: &[Line]) -> Result<Program, String> {
    let mut loop_count = 0;
    let mut labels = vec![];

    let datas = extract_data(lines)?;
    let mut asm_lines = vec![];

    let consume_labels = |labels: &mut Vec<String>| {
        let list = labels.clone();
        labels.clear();
        list
    };
    let gen_target_lbl_for_again = |count: &usize| format!("__loop_{count}_start");
    let gen_lbl_for_loop = |count: &mut usize| {
        *count += 1;
        gen_target_lbl_for_again(count)
    };
    let gen_target_lbl_for_break = |count: &usize| format!("__loop_{count}_end");
    let gen_lbl_for_again = |count: &mut usize| {
        let output = gen_target_lbl_for_break(count);
        *count -= 1;
        output
    };

    for line in lines {
        match line {
            Line::Code {
                line: i,
                label,
                token,
            } => {
                if let Some(lbl) = label {
                    labels.push(lbl.clone());
                }
                match token {
                    Token::Loop => {
                        labels.push(gen_lbl_for_loop(&mut loop_count));
                    }
                    Token::Again => {
                        if loop_count < 1 {
                            return Err(format!(
                                "Found `again` on line {i} but no preceding `loop`"
                            ));
                        }
                        let asm = AsmLine::new(
                            *i,
                            consume_labels(&mut labels),
                            Jump,
                            vec![Label(gen_target_lbl_for_again(&mut loop_count))],
                        );
                        labels.push(gen_lbl_for_again(&mut loop_count));
                        asm_lines.push(asm)
                    }
                    Token::Break => {
                        if loop_count < 1 {
                            return Err(format!(
                                "Found `break` on line {i} but no preceding `loop`"
                            ));
                        }
                        let asm = AsmLine::new(
                            *i,
                            consume_labels(&mut labels),
                            Jump,
                            vec![Label(gen_target_lbl_for_break(&mut loop_count))],
                        );
                        asm_lines.push(asm)
                    }
                    Token::MacroCall(p1, _) => panic!(
                        "Found macro call to {p1} when building opcodes, please raise an issue"
                    ),
                    Token::If(p1, p2) => {
                        match p1 {
                            Condition::Eq(negated, p1, p2) => {
                                let op = match (negated, p1, p2) {
                                    (true, Reg(_), Reg(_)) => SkipIfEqualReg,
                                    (false, Reg(_), Reg(_)) => SkipIfNotEqualReg,
                                    (true, Reg(_), Num(_)) => SkipIfEqualNum,
                                    (false, Reg(_), Num(_)) => SkipIfNotEqualNum,
                                    (_, _, _) => panic!("Invalid params {:?}, {:?} encountered for If Eq when building opcodes, please raise an issue", p1, p2)
                                };
                                asm_lines.push(AsmLine::new(
                                    *i,
                                    consume_labels(&mut labels),
                                    op,
                                    vec![p1.clone(), p2.clone()],
                                ))
                            }
                            Condition::Pressed(negated, p) => {
                                let op = if *negated {
                                    SkipIfKeyPressed
                                } else {
                                    SkipIfKeyNotPressed
                                };
                                asm_lines.push(AsmLine::new(
                                    *i,
                                    consume_labels(&mut labels),
                                    op,
                                    vec![p.clone()],
                                ))
                            }
                        }
                        match p2.as_ref() {
                            Token::Break => {
                                if loop_count < 1 {
                                    return Err(format!(
                                        "Found `break` on line {i} but no preceding `loop`"
                                    ));
                                }
                                let asm = AsmLine::new(
                                    *i,
                                    consume_labels(&mut labels),
                                    Jump,
                                    vec![Label(gen_target_lbl_for_break(&mut loop_count))],
                                );
                                asm_lines.push(asm)
                            },
                            _ => asm_lines.push(get_opcode(*i, vec![], p2)),
                        }
                    }
                    Token::MacroStart(_, _) => {
                        panic!("Found macro start when building opcodes, please raise an issue")
                    }
                    Token::MacroEnd => {
                        panic!("Found macro end when building opcodes, please raise an issue")
                    }
                    _ => asm_lines.push(get_opcode(*i, consume_labels(&mut labels), token)),
                }
            }
            Line::Label { line: _, name } => {
                labels.push(name.clone());
            }
            _ => {}
        }
    }

    if loop_count > 0 {
        return Err(format!("{loop_count} loops not finished at end of program"));
    }

    if !labels.is_empty() {
        return if labels[0].starts_with("__loop") {
            Err(format!("again is not allowed as the last instruction"))
        } else {
            Err(format!("{} unused labels at end of program", labels.len()))
        }
    }

    Ok(Program { datas, asm_lines })
}

fn get_opcode(i: usize, labels: Vec<String>, token: &Token) -> AsmLine {
    match token {
        Token::Return => AsmLine::new(i, labels, Return, vec![]),
        Token::Clear => AsmLine::new(i, labels, ClearDisplay, vec![]),
        Token::Add(p1, p2) => {
            match (p1, p2) {
                (Reg(_), Reg(_)) => AsmLine::new(i, labels, AddReg, vec![p1.clone(), p2.clone()]),
                (Reg(_), Num(_)) => AsmLine::new(i, labels, AddNumToReg, vec![p1.clone(), p2.clone()]),
                (MemReg, Reg(_)) => AsmLine::new(i, labels, AddMemReg, vec![p2.clone()]),
                (_, _) => panic!("Invalid params {:?}, {:?} encountered for Add when building opcodes, please raise an issue", p1, p2)
            }
        }
        Token::Sub(p1, p2) => AsmLine::new(i, labels, SubLeftReg, vec![p1.clone(), p2.clone()]),
        Token::Subr(p1, p2) => AsmLine::new(i, labels, SubRightReg, vec![p1.clone(), p2.clone()]),
        Token::Or(p1, p2) => AsmLine::new(i, labels, BitwiseOr, vec![p1.clone(), p2.clone()]),
        Token::Xor(p1, p2) => AsmLine::new(i, labels, BitwiseXor, vec![p1.clone(), p2.clone()]),
        Token::And(p1, p2) => AsmLine::new(i, labels, BitwiseAnd, vec![p1.clone(), p2.clone()]),
        Token::Set(p1, p2) => {
            match (p1, p2) {
                (Reg(_), Reg(_)) => AsmLine::new(i, labels, SetRegFromReg, vec![p1.clone(), p2.clone()]),
                (Reg(_), Num(_)) => AsmLine::new(i, labels, SetRegFromNum, vec![p1.clone(), p2.clone()]),
                (MemReg, Addr(_)) | (MemReg, Data(_)) | (MemReg, Label(_)) | (MemReg, Unknown(_)) => AsmLine::new(i, labels, SetMemReg, vec![p2.clone()]),
                (_, _) => panic!("Invalid params {:?}, {:?} encountered for Assign when building opcodes, please raise an issue", p1, p2)
            }
        }
        Token::Shr(p) => AsmLine::new(i, labels, ShiftRight, vec![p.clone()]),
        Token::Shl(p) => AsmLine::new(i, labels, ShiftLeft, vec![p.clone()]),
        Token::WaitForKey(p) => AsmLine::new(i, labels, WaitForKey, vec![p.clone()]),
        Token::Rand(p1, p2) => AsmLine::new(i, labels, SetRegRand, vec![p1.clone(), p2.clone()]),
        Token::Draw(p1, p2, p3) => AsmLine::new(
            i,
            labels,
            DrawSprite,
            vec![p1.clone(), p2.clone(), p3.clone()],
        ),
        Token::StoreReg(p) => AsmLine::new(i, labels, StoreRegs, vec![p.clone()]),
        Token::LoadReg(p) => AsmLine::new(i, labels, LoadRegs, vec![p.clone()]),
        Token::Bcd(p) => AsmLine::new(i, labels, StoreBcd, vec![p.clone()]),
        Token::Goto(p) => AsmLine::new(i, labels, Jump, vec![p.clone()]),
        Token::GotoOffset(p1, p2) => {
            AsmLine::new(i, labels, JumpOffset, vec![p1.clone(), p2.clone()])
        }
        Token::Digit(p) => AsmLine::new(i, labels, SetMemRegToDigitSprite, vec![p.clone()]),
        Token::Ascii(p) => AsmLine::new(i, labels, SetMemRegToAsciiSprite, vec![p.clone()]),
        Token::Call(p) => AsmLine::new(i, labels, Call, vec![p.clone()]),
        _ => panic!(
            "Unhandled instruction {:?} (line {i}) when building opcodes, please raise an issue",
            token
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_get_opcode() {
        assert_eq!(
            get_opcode(6, vec![], &Token::Bcd(Reg(0))),
            AsmLine::new(6, vec![], StoreBcd, vec![Reg(0)])
        );
        assert_eq!(
            get_opcode(10, vec!["lbl".to_string()], &Token::Add(Reg(10), Num(5))),
            AsmLine::new(
                10,
                vec!["lbl".to_string()],
                AddNumToReg,
                vec![Reg(10), Num(5)]
            )
        );
    }
}
