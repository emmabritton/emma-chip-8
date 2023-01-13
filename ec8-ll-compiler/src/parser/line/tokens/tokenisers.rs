use crate::parser::line::models::Definition;
use crate::parser::line::tokens::util::{cleanup, parse_param};
use crate::parser::line::tokens::Condition::{Eq, Pressed};
use crate::parser::line::tokens::Token::*;
use crate::parser::line::tokens::{tokenise, Condition, MacroParam, Token};

pub fn tokenise_if(i: usize, parts: &[String], defs: &[Definition]) -> Result<Token, String> {
    let mut parts = parts.iter().map(|str| str.as_str()).collect::<Vec<&str>>();
    if let Some((arg, remaining)) = parts[1].split_once('(') {
        parts.remove(1);
        parts.insert(1, remaining);
        parts.insert(1, arg);
    }
    let mut cond_keyword = parts[1];
    let mut params = cleanup(&parts.iter().skip(2).copied().collect::<Vec<&str>>());
    let mut negate = false;
    if cond_keyword.starts_with('!') {
        negate = true;
        cond_keyword = cond_keyword.trim_start_matches('!');
    }

    let cond = parse_conditional(cond_keyword, negate, &mut params, defs)?;

    let (label, token) = tokenise(i, &params.join(" "), defs);
    if label.is_some() {
        return Err("label not allowed after if".to_string());
    }
    match token {
        Ok(token) => {
            if matches!(
                token,
                If(_, _) | MacroEnd | MacroCall(_, _) | MacroStart(_, _) | Loop | Again
            ) {
                return Err(format!("{:?} not allowed after if", token));
            }
            Ok(If(cond, Box::new(token)))
        }
        Err(err) => Err(format!("Error parsing op after if: {err}")),
    }
}

pub fn parse_conditional(
    cond_keyword: &str,
    negate: bool,
    params: &mut Vec<String>,
    defs: &[Definition],
) -> Result<Condition, String> {
    match cond_keyword.trim() {
        "eq" => {
            if params.len() < 2 {
                return Err("if eq() requires two params".to_string());
            }
            Ok(Eq(
                negate,
                parse_param(&params.remove(0), defs)?,
                parse_param(&params.remove(0), defs)?,
            ))
        }
        "pressed" => {
            if params.is_empty() {
                return Err("if pressed() requires one param".to_string());
            }
            Ok(Pressed(negate, parse_param(&params.remove(0), defs)?))
        }
        _ => Err("Unknown cond for if: {cond_keyword}".to_string()),
    }
}

pub fn tokenise_one_param(parts: &[String], defs: &[Definition]) -> Result<Token, String> {
    if parts.len() != 2 {
        return Err(format!("{} requires one param", parts[0]));
    }
    let param = parse_param(&parts[1], defs)?;
    match parts[0].as_str() {
        "shr" => Ok(Shr(param)),
        "shl" => Ok(Shl(param)),
        "digit" => Ok(Digit(param)),
        "ascii" => Ok(Ascii(param)),
        "call" => Ok(Call(param)),
        "bcd" => Ok(Bcd(param)),
        "wait_for_key" => Ok(WaitForKey(param)),
        "reg_load" => Ok(LoadReg(param)),
        "reg_store" => Ok(StoreReg(param)),
        _ => panic!(
            "Failed match keyword for one param for {}, please raise an issue",
            parts[0]
        ),
    }
}

pub fn tokenise_two_param(parts: &[String], defs: &[Definition]) -> Result<Token, String> {
    if parts.len() != 3 {
        return Err(format!("{} requires two params", parts[0]));
    }
    let param1 = parse_param(&parts[1], defs)?;
    let param2 = parse_param(&parts[2], defs)?;
    match parts[0].as_str() {
        "rand" => Ok(Rand(param1, param2)),
        _ => panic!(
            "Failed match keyword for two param for {}, please raise an issue",
            parts[0]
        ),
    }
}

pub fn tokenise_three_param(parts: &[String], defs: &[Definition]) -> Result<Token, String> {
    if parts.len() != 4 {
        return Err(format!("{} requires three params", parts[0]));
    }
    let param1 = parse_param(&parts[1], defs)?;
    let param2 = parse_param(&parts[2], defs)?;
    let param3 = parse_param(&parts[3], defs)?;
    match parts[0].as_str() {
        "draw" => Ok(Draw(param1, param2, param3)),
        _ => panic!(
            "Failed match keyword for three param for {}, please raise an issue",
            parts[0]
        ),
    }
}

pub fn tokenise_one_two_param(parts: &[String], defs: &[Definition]) -> Result<Token, String> {
    if parts.len() == 2 {
        let param = parse_param(&parts[1], defs)?;
        match parts[0].as_str() {
            "goto" => Ok(Goto(param)),
            _ => panic!(
                "Failed match keyword for one/two param for {} (1), please raise an issue",
                parts[0]
            ),
        }
    } else if parts.len() == 3 {
        let param1 = parse_param(&parts[1], defs)?;
        let param2 = parse_param(&parts[2], defs)?;
        match parts[0].as_str() {
            "goto" => Ok(GotoOffset(param1, param2)),
            _ => panic!(
                "Failed match keyword for one/two param for {} (2), please raise an issue",
                parts[0]
            ),
        }
    } else {
        return Err(format!("{} requires one or two params", parts[0]));
    }
}

pub fn tokenise_math(parts: &[String], defs: &[Definition]) -> Result<Option<Token>, String> {
    if parts.len() < 3 {
        return Ok(None);
    }
    let lhs = parse_param(&parts[0], defs)?;
    let rhs = parse_param(&parts[2], defs)?;
    return match parts[1].as_str() {
        "=" => {
            if parts.len() == 5 && parts[3] == "-" {
                if parts[0] != parts[4] {
                    return Err("Target and subtrahend must be the same".to_string());
                }
                Ok(Some(Subr(lhs, rhs)))
            } else {
                if parts.len() != 3 {
                    return Err("Unable to parse assign".to_string());
                }
                Ok(Some(Set(lhs, rhs)))
            }
        }
        "+=" => {
            if parts.len() != 3 {
                return Err("Unable to parse add".to_string());
            }
            Ok(Some(Add(lhs, rhs)))
        }
        "-=" => {
            if parts.len() != 3 {
                return Err("Unable to parse add".to_string());
            }
            Ok(Some(Sub(lhs, rhs)))
        }
        "|=" => {
            if parts.len() != 3 {
                return Err("Unable to parse add".to_string());
            }
            Ok(Some(Or(lhs, rhs)))
        }
        "&=" => {
            if parts.len() != 3 {
                return Err("Unable to parse add".to_string());
            }
            Ok(Some(And(lhs, rhs)))
        }
        "^=" => {
            if parts.len() != 3 {
                return Err("Unable to parse add".to_string());
            }
            Ok(Some(Xor(lhs, rhs)))
        }
        _ => Ok(None),
    };
}

pub fn tokenise_macro_call(parts: &[String], defs: &[Definition]) -> Result<Token, String> {
    let mut iter = parts.iter();
    let name = iter.next().unwrap().trim_end_matches('!');
    let mut params = vec![];
    for part in iter {
        params.push(parse_param(part, defs)?);
    }
    Ok(MacroCall(name.to_string(), params))
}

pub fn tokenise_macro_def(parts: &[String]) -> Result<Token, String> {
    let mut iter = parts.iter().skip(1);
    let name = iter
        .next()
        .unwrap()
        .trim_matches(|c| matches!(c, ',' | '('));
    let mut params = vec![];
    for part in iter {
        params.push(parse_macro_param(part)?);
    }
    Ok(MacroStart(name.to_string(), params))
}

pub fn parse_macro_param(param: &str) -> Result<MacroParam, String> {
    match param.trim().trim_matches(|c| matches!(c, '(' | ')' | ',')) {
        "r" => Ok(MacroParam::Reg),
        "l" => Ok(MacroParam::Label),
        "n" => Ok(MacroParam::Nibble),
        "nn" => Ok(MacroParam::Num),
        "a" => Ok(MacroParam::Addr),
        "la" => Ok(MacroParam::LabelAddr),
        "d" => Ok(MacroParam::Data),
        "da" => Ok(MacroParam::DataAddr),
        _ => Err(format!("Unable to parse macro param {param}")),
    }
}

pub fn tokenise_no_param(parts: &[String]) -> Result<Token, String> {
    if parts.len() > 1 {
        return Err(format!("{} doesn't take params", parts[0]));
    }
    let token = match parts[0].as_str() {
        "loop" => Loop,
        "again" => Again,
        "break" => Break,
        "return" => Return,
        "clear" => Clear,
        "end" => MacroEnd,
        _ => panic!("Found {} inside match arg, please raise an issue", parts[0]),
    };
    Ok(token)
}
