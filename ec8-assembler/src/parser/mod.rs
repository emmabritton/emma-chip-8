use crate::program::{Line, Program};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use ec8_common::nibbler::Nibbler;
use ec8_common::OpCodes;
use ec8_common::OpCodes::*;

pub fn parse(source: Vec<&str>) -> Result<Program> {
    let source = clean_up(source);

    let mut lines = vec![];
    for (i, code, comment) in source {
        if code.is_empty() {
            lines.push(Line::new_comment(i, &comment));
        } else {
            lines.push(
                parse_line(i, &code)
                    .map_err(|txt| eyre!(txt))?
                    .append_comment(&comment),
            );
        }
    }

    Ok(Program::new(lines))
}

fn clean_up(source: Vec<&str>) -> Vec<(usize, String, String)> {
    source
        .iter()
        .map(|line| {
            line.split_once(';')
                .map(|(source, comment)| (source.to_string(), comment.to_string()))
                .unwrap_or((line.to_string(), String::new()))
        })
        .enumerate()
        .filter(|(_, (code, comment))| !code.is_empty() || !comment.is_empty())
        .map(|(i, (source, comment))| (i, source.trim().to_string(), comment))
        .collect()
}

fn parse_line(i: usize, line: &str) -> Result<Line, String> {
    let line = line.trim();
    if line.chars().count() < 3 {
        return Err(format!("Line {i} is invalid"));
    }
    let (op, params) = line.split_at(3);
    match op.to_lowercase().trim() {
        "clr" => Ok(Line::no_params(i, ClearDisplay, [0x00, 0xE0])),
        "ret" => Ok(Line::no_params(i, Return, [0x00, 0xEE])),
        "jmp" => Line::nnn(i, Jump, 0x10, params),
        "cal" => Line::nnn(i, Call, 0x20, params),
        "ske" => Line::xnn_xy(i, (SkipIfEqualNum, 0x30), (SkipIfEqualReg, 0x50, 0), params),
        "skn" => Line::xnn_xy(
            i,
            (SkipIfNotEqualNum, 0x40),
            (SkipIfNotEqualReg, 0x90, 0),
            params,
        ),
        "set" => Line::xnn_xy(i, (SetRegFromNum, 0x60), (SetRegFromReg, 0x80, 0), params),
        "add" => Line::xnn_xy(i, (AddNumToReg, 0x70), (AddReg, 0x80, 4), params),
        "or" => Line::xy(i, BitwiseOr, 0x80, 1, params),
        "and" => Line::xy(i, BitwiseAnd, 0x80, 2, params),
        "xor" => Line::xy(i, BitwiseXor, 0x80, 3, params),
        "sub" => Line::xy(i, SubRightReg, 0x80, 5, params),
        "shr" => Line::xy(i, ShiftRight, 0x80, 6, params),
        "sbr" => Line::xy(i, SubLeftReg, 0x80, 7, params),
        "shl" => Line::xy(i, ShiftLeft, 0x80, 0xE, params),
        "sti" => Line::nnn(i, SetMemReg, 0xA0, params),
        "jp0" => Line::nnn(i, JumpOffset, 0xB0, params),
        "rnd" => Line::xnn(i, SetRegRand, 0xC0, params),
        "drw" => Line::xyn(i, DrawSprite, 0xD0, params),
        "skp" => Line::x(i, SkipIfKeyPressed, 0xE0, 0x9E, params),
        "skr" => Line::x(i, SkipIfKeyNotPressed, 0xE0, 0xA1, params),
        "rdt" => Line::x(i, SetRegFromTimer, 0xF0, 0x07, params),
        "key" => Line::x(i, WaitForKey, 0xF0, 0x0A, params),
        "sdt" => Line::x(i, SetDelayTimer, 0xF0, 0x15, params),
        "sst" => Line::x(i, SetSoundTimer, 0xF0, 0x18, params),
        "adi" => Line::x(i, AddMemReg, 0xF0, 0x1E, params),
        "chr" => Line::x(i, SetMemRegToDigitSprite, 0xF0, 0x29, params),
        "bcd" => Line::x(i, StoreBcd, 0xF0, 0x33, params),
        "str" => Line::x(i, StoreRegs, 0xF0, 0x55, params),
        "ldr" => Line::x(i, LoadRegs, 0xF0, 0x65, params),
        _ => Err(format!("Line {i}) mnemonic {op} is unknown")),
    }
}

impl Line {
    pub fn no_params(i: usize, opcode: OpCodes, bytes: [u8; 2]) -> Self {
        Line::new_code(i, opcode, bytes)
    }

    pub fn x(i: usize, opcode: OpCodes, first: u8, last: u8, params: &str) -> Result<Self, String> {
        let vx = parse_reg(params, 1).map_err(|err| format!("Line {i}) {err}"))?;

        let bytes = [first | vx, last];
        Ok(Line::no_params(i, opcode, bytes))
    }

    pub fn xy(
        i: usize,
        opcode: OpCodes,
        first: u8,
        last: u8,
        params: &str,
    ) -> Result<Self, String> {
        let (vx, vy) = params
            .split_once(',')
            .ok_or(format!("Line {i}) Two registers required"))?;
        let vx = parse_reg(vx, 1).map_err(|err| format!("Line {i}) {err}"))?;
        let vy = parse_reg(vy, 2).map_err(|err| format!("Line {i}) {err}"))?;

        let bytes = [first | vx, last | vy];
        Ok(Line::no_params(i, opcode, bytes))
    }

    pub fn nnn(i: usize, opcode: OpCodes, first: u8, addr_param: &str) -> Result<Self, String> {
        let addr_param = addr_param.trim();
        if addr_param.chars().count() > 3 {
            return Err(format!("Line {i}) Address param is too long"));
        }
        match u16::from_str_radix(addr_param, 16) {
            Ok(addr) => {
                let addr = addr.to_be_bytes();
                let bytes = [first | addr[0].second_nibble(), addr[1]];
                Ok(Line::no_params(i, opcode, bytes))
            }
            Err(err) => Err(format!("Line {i}) Unable to parse address {err}")),
        }
    }

    pub fn xyn(i: usize, opcode: OpCodes, first: u8, params: &str) -> Result<Line, String> {
        let params = params.to_lowercase();
        let params = params
            .split(',')
            .map(|str| str.trim())
            .collect::<Vec<&str>>();
        if params.len() != 3 {
            return Err(format!("Line {i}) Three params required"));
        }
        let x = parse_reg(params[0], 1)?;
        let y = parse_reg(params[1], 2)?;
        if params[2].chars().count() > 1 {
            return Err(format!("Line {i}) Number param is too long"));
        }
        match u8::from_str_radix(params[2], 16) {
            Ok(num) => {
                let bytes = [first | x, y | num];
                Ok(Line::no_params(i, opcode, bytes))
            }
            Err(err) => Err(format!("Line {i}) Unable to parse number {err}")),
        }
    }

    pub fn xnn(i: usize, opcode: OpCodes, first: u8, params: &str) -> Result<Line, String> {
        let params = params.to_lowercase();
        let (x, nn) = params
            .split_once(',')
            .ok_or(format!("Line {i}) Two params required"))?;
        let x = parse_reg(x, 1)?;
        let nn = nn.trim();
        if nn.chars().count() > 2 {
            return Err(format!("Line {i}) Number param is too long"));
        }
        match u8::from_str_radix(nn, 16) {
            Ok(num) => {
                let bytes = [first | x, num];
                Ok(Line::no_params(i, opcode, bytes))
            }
            Err(err) => Err(format!("Line {i}) Unable to parse number {err}")),
        }
    }

    pub fn xnn_xy(
        i: usize,
        (xnn_opcode, xnn_first): (OpCodes, u8),
        (xy_opcode, xy_first, xy_last): (OpCodes, u8, u8),
        params: &str,
    ) -> Result<Line, String> {
        let params = params.to_lowercase();
        let (x, nn_y) = params
            .split_once(',')
            .ok_or(format!("Line {i}) Two params required"))?;
        let x = parse_reg(x, 1)?;
        let nn_y = nn_y.trim();
        match nn_y.contains('v') {
            true => {
                let y = parse_reg(nn_y, 2)?;
                let bytes = [xy_first | x, xy_last | y];
                Ok(Line::no_params(i, xy_opcode, bytes))
            }
            false => {
                if nn_y.chars().count() > 2 {
                    return Err(format!("Line {i}) Number param is too long"));
                }
                match u8::from_str_radix(nn_y, 16) {
                    Ok(num) => {
                        let bytes = [xnn_first | x, num];
                        Ok(Line::no_params(i, xnn_opcode, bytes))
                    }
                    Err(err) => Err(format!("Line {i}) Unable to parse number {err}")),
                }
            }
        }
    }
}

fn parse_reg(reg: &str, which: usize) -> Result<u8, String> {
    let reg = reg.trim().to_lowercase();
    if reg.chars().count() != 2 {
        return Err(format!("Reg {which} is invalid"));
    }
    if !reg.starts_with('v') {
        return Err(format!("Reg {which} is invalid"));
    }
    let digit = u8::from_str_radix(reg.chars().skip(1).take(1).collect::<String>().as_str(), 16)
        .map_err(|err| format!("Unable to parse reg {which}: {err}"))?;
    match which {
        1 => Ok(digit),
        2 => Ok(digit << 4),
        _ => panic!("Invalid parse_reg which {which}"),
    }
}

#[cfg(test)]
mod test {
    use crate::parser::{clean_up, parse, parse_line, parse_reg};
    use crate::program::Line;
    use ec8_common::OpCodes::*;

    #[test]
    fn check_parse() {
        let source = vec!["CLR", "RET", "JMP 123", "ADD V0, ve"];
        let program = parse(source).unwrap();
        assert_eq!(program.describe(), "00E0 Clear the display \n00EE Return from subroutine \n1123 Jump to 123 \n80E4 Set V0 to V0 + VE \n".to_string());
        assert_eq!(
            program.into_bytes(),
            vec![0x00, 0xE0, 0x00, 0xEE, 0x11, 0x23, 0x80, 0xE4]
        );

        let source = vec![";test", "CLR", "RET;no ret", "JMP 123", "ADD V0, ve"];
        let program = parse(source).unwrap();
        assert_eq!(program.describe(), ";test\n00E0 Clear the display \n00EE Return from subroutine ;no ret\n1123 Jump to 123 \n80E4 Set V0 to V0 + VE \n".to_string());
        assert_eq!(
            program.into_bytes(),
            vec![0x00, 0xE0, 0x00, 0xEE, 0x11, 0x23, 0x80, 0xE4]
        );
    }

    #[test]
    fn check_parse_line() {
        assert_eq!(
            parse_line(4, "JMP 41A"),
            Ok(Line::no_params(4, Jump, [0x14, 0x1A]))
        );
        assert_eq!(
            parse_line(6, " XOR  V3 , va"),
            Ok(Line::no_params(6, BitwiseXor, [0x83, 0xA3]))
        );
    }

    #[test]
    fn check_clean_up() {
        let prog = "ASM 1\nASM 2";
        let cleaned = clean_up(prog.lines().collect());
        assert_eq!(
            cleaned,
            vec![
                (0, "ASM 1".to_string(), "".to_string()),
                (1, "ASM 2".to_string(), "".to_string())
            ]
        );

        let prog = "\nASM 1\n;whole line\nASM 2;note";
        let cleaned = clean_up(prog.lines().collect());
        assert_eq!(
            cleaned,
            vec![
                (1, "ASM 1".to_string(), "".to_string()),
                (2, "".to_string(), "whole line".to_string()),
                (3, "ASM 2".to_string(), "note".to_string())
            ]
        );
    }

    #[test]
    fn check_no_params() {
        assert_eq!(
            Line::no_params(5, ClearDisplay, [45, 67]),
            Line::new_code(5, ClearDisplay, [45, 67])
        )
    }

    #[test]
    fn check_x() {
        assert_eq!(
            Line::x(0, AddMemReg, 0xF0, 0x1E, "v4"),
            Ok(Line::new_code(0, AddMemReg, [0xF4, 0x1E]))
        );
        assert_eq!(
            Line::x(4, AddMemReg, 0xF0, 0x1E, ""),
            Err("Line 4) Reg 1 is invalid".to_string())
        );
        assert_eq!(
            Line::x(8, AddMemReg, 0xF0, 0x1E, "12"),
            Err("Line 8) Reg 1 is invalid".to_string())
        );
        assert_eq!(
            Line::x(99, AddMemReg, 0xF0, 0x1E, "vp"),
            Err("Line 99) Unable to parse reg 1: invalid digit found in string".to_string())
        );
    }

    #[test]
    fn check_xnn() {
        assert_eq!(
            Line::xnn(53, SkipIfEqualNum, 0x30, "VB , 18"),
            Ok(Line::new_code(53, SkipIfEqualNum, [0x3B, 0x18]))
        );
        assert_eq!(
            Line::xnn(54, SkipIfNotEqualNum, 0x40, "VB , 181"),
            Err("Line 54) Number param is too long".to_string())
        );
    }

    #[test]
    fn check_nnn() {
        assert_eq!(
            Line::nnn(3, Jump, 0x10, "1ad"),
            Ok(Line::new_code(3, Jump, [0x11, 0xAD]))
        );
        assert_eq!(
            Line::nnn(6, Call, 0x20, "v1"),
            Err("Line 6) Unable to parse address invalid digit found in string".to_string())
        );
        assert_eq!(
            Line::nnn(7, SetMemReg, 0xA0, "1234"),
            Err("Line 7) Address param is too long".to_string())
        );
    }

    #[test]
    fn check_xy() {
        assert_eq!(
            Line::xy(1, AddReg, 0x80, 0x04, "v4, va"),
            Ok(Line::new_code(1, AddReg, [0x84, 0xA4]))
        );
        assert_eq!(
            Line::xy(10, BitwiseOr, 0x80, 0x01, "v1"),
            Err("Line 10) Two registers required".to_string())
        );
        assert_eq!(
            Line::xy(12, BitwiseAnd, 0x80, 0x02, ", v2"),
            Err("Line 12) Reg 1 is invalid".to_string())
        );
        assert_eq!(
            Line::xy(9, BitwiseXor, 0x80, 0x03, "1, 3"),
            Err("Line 9) Reg 1 is invalid".to_string())
        );
    }

    #[test]
    fn check_xyn() {
        assert_eq!(
            Line::xyn(1, DrawSprite, 0xD0, "v4, va, 6"),
            Ok(Line::new_code(1, DrawSprite, [0xD4, 0xA6]))
        );
        assert_eq!(
            Line::xyn(2, DrawSprite, 0xD0, "v1"),
            Err("Line 2) Three params required".to_string())
        );
    }

    #[test]
    fn check_parse_reg() {
        assert_eq!(parse_reg("v3", 1), Ok(0x03));
        assert_eq!(parse_reg("v3", 2), Ok(0x30));

        assert_eq!(parse_reg("vF", 1), Ok(0x0F));
        assert_eq!(parse_reg("vA", 2), Ok(0xA0));

        assert!(parse_reg("1", 1).is_err());
        assert!(parse_reg("V11", 1).is_err());
        assert!(parse_reg("Vp", 1).is_err());
        assert!(parse_reg("V", 1).is_err());
    }
}
