mod tokenisers;
mod util;
mod validators;

use crate::parser::line::models::Definition;
use crate::parser::line::tokens::tokenisers::*;
use crate::parser::line::tokens::util::cleanup;
use crate::parser::line::tokens::Token::*;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Token {
    Loop,
    Again,
    Break,
    Return,
    Clear,
    MacroCall(String, Vec<Param>),
    Add(Param, Param),
    Sub(Param, Param),
    Subr(Param, Param),
    Or(Param, Param),
    Xor(Param, Param),
    And(Param, Param),
    Set(Param, Param),
    Shr(Param),
    Shl(Param),
    WaitForKey(Param),
    Rand(Param, Param),
    Draw(Param, Param, Param),
    StoreReg(Param),
    LoadReg(Param),
    Bcd(Param),
    If(Condition, Box<Token>),
    Goto(Param),
    GotoOffset(Param, Param),
    Digit(Param),
    Ascii(Param),
    Call(Param),
    MacroStart(String, Vec<MacroParam>),
    MacroEnd,
}

impl Token {
    pub fn replace_placeholders(&self, params: &[Param]) -> Token {
        match self {
            MacroEnd | Clear | Return | Loop | Again | Break | MacroStart(_, _) => self.clone(),
            MacroCall(name, macro_params) => MacroCall(
                name.clone(),
                macro_params
                    .iter()
                    .map(|param| replace(param, params))
                    .collect(),
            ),
            Add(p1, p2) => Add(replace(p1, params), replace(p2, params)),
            Sub(p1, p2) => Sub(replace(p1, params), replace(p2, params)),
            Subr(p1, p2) => Subr(replace(p1, params), replace(p2, params)),
            Or(p1, p2) => Or(replace(p1, params), replace(p2, params)),
            Xor(p1, p2) => Xor(replace(p1, params), replace(p2, params)),
            And(p1, p2) => And(replace(p1, params), replace(p2, params)),
            Set(p1, p2) => Set(replace(p1, params), replace(p2, params)),
            Shr(p1) => Shr(replace(p1, params)),
            Shl(p1) => Shl(replace(p1, params)),
            WaitForKey(p1) => WaitForKey(replace(p1, params)),
            Rand(p1, p2) => Rand(replace(p1, params), replace(p2, params)),
            Draw(p1, p2, p3) => Draw(
                replace(p1, params),
                replace(p2, params),
                replace(p3, params),
            ),
            StoreReg(p1) => StoreReg(replace(p1, params)),
            LoadReg(p1) => LoadReg(replace(p1, params)),
            Bcd(p1) => Bcd(replace(p1, params)),
            If(p1, p2) => If(p1.clone(), Box::new(p2.replace_placeholders(params))),
            Goto(p1) => Goto(replace(p1, params)),
            GotoOffset(p1, p2) => GotoOffset(replace(p1, params), replace(p2, params)),
            Digit(p1) => Digit(replace(p1, params)),
            Ascii(p1) => Ascii(replace(p1, params)),
            Call(p1) => Call(replace(p1, params)),
        }
    }
}

fn replace(param: &Param, placeholders: &[Param]) -> Param {
    if let Param::Placeholder(idx) = param {
        &placeholders[*idx as usize - 1]
    } else {
        param
    }
    .clone()
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Condition {
    Eq(bool, Param, Param),
    Pressed(bool, Param),
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum MacroParam {
    Reg,
    Nibble,
    Num,
    Label,
    Data,
    Addr,
    LabelAddr,
    DataAddr,
}

impl MacroParam {
    pub fn check_compat(&self, line: usize, idx: usize, param: &Param) -> Result<(), String> {
        match self {
            MacroParam::Reg => match param {
                Param::Reg(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires register for param {idx}"
                )),
            },
            MacroParam::Nibble => match param {
                Param::Num(num) => {
                    if *num < 16 {
                        Ok(())
                    } else {
                        Err(format!("Macro call on line {line} requires nibble (number too large) for param {idx}"))
                    }
                }
                _ => Err(format!(
                    "Macro call on line {line} requires nibble for param {idx}"
                )),
            },
            MacroParam::Num => match param {
                Param::Num(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires number for param {idx}"
                )),
            },
            MacroParam::Label => match param {
                Param::Label(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires label for param {idx}"
                )),
            },
            MacroParam::Data => match param {
                Param::Data(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires data for param {idx}"
                )),
            },
            MacroParam::Addr => match param {
                Param::Addr(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires addr for param {idx}"
                )),
            },
            MacroParam::LabelAddr => match param {
                Param::Label(_) | Param::Addr(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires label or addr for param {idx}"
                )),
            },
            MacroParam::DataAddr => match param {
                Param::Data(_) | Param::Addr(_) => Ok(()),
                _ => Err(format!(
                    "Macro call on line {line} requires data or addr for param {idx}"
                )),
            },
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Param {
    Reg(u8),
    Placeholder(u8),
    Addr(u16),
    Num(u8),
    Sound,
    Delay,
    MemReg,
    Label(String),
    Data(String),
    Unknown(String),
}

pub fn tokenise(
    i: usize,
    line: &str,
    defs: &[Definition],
) -> (Option<String>, Result<Token, String>) {
    let parts = line.split(|c: char| c.is_whitespace() || c == ',').collect::<Vec<&str>>();

    if parts.is_empty() {
        panic!("Found empty string {i} when trying to tokenise, please raise an issue");
    }

    let mut parts = cleanup(&parts);

    let label = if parts[0].ends_with(':') {
        Some(parts.remove(0).trim_end_matches(':').to_string())
    } else {
        None
    };

    parts[0] = parts[0].to_lowercase();

    if let Some((arg, remaining)) = parts[0].clone().split_once('(') {
        parts.remove(0);
        parts.insert(0, remaining.to_string());
        parts.insert(0, arg.to_string());
    }

    if parts[0].ends_with('!') {
        return (label, tokenise_macro_call(&parts, defs));
    }

    match parts[0].as_str() {
        "loop" | "again" | "break" | "return" | "clear" | "end" => {
            return (label, tokenise_no_param(&parts));
        }
        "reg_store" | "reg_load" | "bcd" | "shr" | "shl" | "digit" | "ascii" | "wait_for_key"
        | "call" => return (label, tokenise_one_param(&parts, defs)),
        "rand" => return (label, tokenise_two_param(&parts, defs)),
        "draw" => return (label, tokenise_three_param(&parts, defs)),
        "goto" => return (label, tokenise_one_two_param(&parts, defs)),
        "macro" => return (label, tokenise_macro_def(&parts)),
        "if" => return (label, tokenise_if(i, &parts, defs)),
        _ => {}
    }

    match tokenise_math(&parts, defs) {
        Ok(result) => {
            if let Some(token) = result {
                return (label, Ok(token));
            }
        }
        Err(err) => return (None, Err(err)),
    }

    (None, Err(format!("Unable to parse line {i}, unknown instruction '{line}'")))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::line::models::DefinitionType;
    use crate::parser::line::tokens::util::parse_param;
    use crate::parser::line::tokens::Param::*;

    fn token(line: &str, defs: &[Definition], token: Token, label: Option<&str>) {
        assert_eq!(
            tokenise(0, line, defs),
            (label.map(|s| s.to_string()), Ok(token))
        );
    }

    fn param(text: &str, defs: &[Definition], value: Param) {
        assert_eq!(parse_param(text, defs), Ok(value))
    }

    fn alias(name: &str, value: &str) -> Definition {
        Definition::new_alias(
            0,
            DefinitionType::Alias,
            name.to_string(),
            value.to_string(),
        )
    }

    fn label(name: &str) -> Definition {
        Definition::new(0, DefinitionType::Label, name.to_string())
    }

    #[test]
    fn basic_checks() {
        token("clear", &[], Clear, None);
        token("test: CLEAR", &[], Clear, Some("test"));
        token("shl(v5)", &[], Shl(Reg(5)), None);
        token("v1 += inc", &[alias("inc", "1")], Add(Reg(1), Num(1)), None);
        token(
            "v2 += inc",
            &[alias("inc", "vf")],
            Add(Reg(2), Reg(15)),
            None,
        );
        token(
            "VA += inc",
            &[alias("inc", "v10")],
            Add(Reg(10), Reg(10)),
            None,
        );
        token(
            "label: lhs = rhs - lhs",
            &[alias("lhs", "i"), alias("rhs", "delay")],
            Subr(MemReg, Delay),
            Some("label"),
        );
        token(
            "draw_digit!(v0, v0, 5)",
            &[],
            MacroCall("draw_digit".to_string(), vec![Reg(0), Reg(0), Num(5)]),
            None,
        );
        token(
            "macro(example, r, la, nn)",
            &[],
            MacroStart(
                "example".to_string(),
                vec![MacroParam::Reg, MacroParam::LabelAddr, MacroParam::Num],
            ),
            None,
        );
    }

    #[test]
    fn whitespace_checks() {
        token("shl ( v5 )", &[], Shl(Reg(5)), None);
        token(
            "draw_digit! ( v0 , v0, v5)",
            &[],
            MacroCall("draw_digit".to_string(), vec![Reg(0), Reg(0), Reg(5)]),
            None,
        );
        token(
            "draw_digit!(vd,v3,v8)",
            &[],
            MacroCall("draw_digit".to_string(), vec![Reg(13), Reg(3), Reg(8)]),
            None,
        );
        token(
            "draw ( vd , v3 , 8 ) ",
            &[],
            Draw(Reg(0xD), Reg(0x3), Num(8)),
            None,
        );
        token(
            "draw(vd,v3,8)",
            &[],
            Draw(Reg(0xD), Reg(0x3), Num(8)),
            None,
        );
    }

    #[test]
    fn check_if() {
        token(
            "if eq(v0, v1) break",
            &[],
            If(Condition::Eq(false, Reg(0), Reg(1)), Box::new(Break)),
            None,
        );
        token(
            "if !pressed($1) $2 += x2",
            &[],
            If(
                Condition::Pressed(true, Placeholder(1)),
                Box::new(Add(Placeholder(2), Num(2))),
            ),
            None,
        );
        token(
            " label: if !pressed ( $1 ) $2 += x2 ",
            &[],
            If(
                Condition::Pressed(true, Placeholder(1)),
                Box::new(Add(Placeholder(2), Num(2))),
            ),
            Some("label"),
        );
    }

    #[test]
    fn check_params() {
        param("label", &[label("label")], Label("label".to_string()));
        param("'a'", &[], Num(97));
        param("v1", &[], Reg(1));
        param("v10", &[], Reg(10));
        param("vf", &[], Reg(15));
        param("i", &[], MemReg);
        param("$1", &[], Placeholder(1));
        param("@123", &[], Addr(123));
        param("@x123", &[], Addr(291));
        param("delay", &[], Delay);
        param("sound", &[], Sound);
        param("12", &[], Num(12));
        param("x12", &[], Num(0x12));
        param("-12", &[], Num(244));
        param("b10110011", &[], Num(179));
    }

    #[test]
    fn check_macro_def() {
        let lines = vec![
            "macro(read_data, l, r)",
            "i = $1",
            "i += $2",
            "v0 = 0",
            "reg_load(v0)",
            "end",
        ];

        let mut tokens = vec![];
        for (i, line) in lines.iter().enumerate() {
            let (_, result) = tokenise(i, line, &[]);
            tokens.push(result.unwrap())
        }

        assert_eq!(
            tokens,
            vec![
                MacroStart(
                    "read_data".to_string(),
                    vec![MacroParam::Label, MacroParam::Reg],
                ),
                Set(MemReg, Placeholder(1)),
                Add(MemReg, Placeholder(2)),
                Set(Reg(0), Num(0)),
                LoadReg(Reg(0)),
                MacroEnd,
            ]
        );
    }
}
