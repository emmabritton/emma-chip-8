use crate::parser::line::models::Definition;
use crate::parser::line::tokens::tokenise;
use crate::parser::line::utils::{check_name, SYMBOLS};
use crate::parser::line::{Expected, Line};
use std::str::FromStr;

pub fn parse_code(i: usize, line: &str, defs: &[Definition]) -> Option<Line> {
    let (label, token) = tokenise(i, line, defs);
    match token {
        Ok(token) => Some(Line::new_code(i, label, token)),
        Err(err) => Some(Line::new_error(i, Expected::Code, err)),
    }
}

pub fn parse_label(i: usize, line: &str, defs: &[Definition]) -> Option<Line> {
    let name = line.trim().trim_end_matches(':');
    match check_name(name, defs) {
        None => Some(Line::new_label(i, name.to_string())),
        Some(err) => Some(Line::new_error(
            i,
            Expected::Label,
            format!("Invalid name: {err}"),
        )),
    }
}

pub fn parse_alias(i: usize, line: &str, defs: &[Definition]) -> Option<Line> {
    let parts = line
        .trim_start_matches("alias")
        .split_whitespace()
        .collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Some(Line::new_error(
            i,
            Expected::Alias,
            "Invalid format".to_string(),
        ));
    }
    match check_name(parts[0], defs) {
        Some(err) => Some(Line::new_error(
            i,
            Expected::Alias,
            format!("Invalid name: {err}"),
        )),
        None => Some(Line::new_alias(
            i,
            parts[0].trim().to_string(),
            parts[1].trim().to_string(),
        )),
    }
}

pub fn parse_data(i: usize, line: &str, defs: &[Definition]) -> Option<Line> {
    let parts = line
        .trim_start_matches("data")
        .split_whitespace()
        .collect::<Vec<&str>>();
    if parts.len() != 2 {
        return Some(Line::new_error(
            i,
            Expected::Data,
            "Invalid format".to_string(),
        ));
    }
    if let Some(err) = check_name(parts[0], defs) {
        return Some(Line::new_error(
            i,
            Expected::Data,
            format!("Invalid name: {err}"),
        ));
    }
    match parse_data_value(parts[1]) {
        Ok(data) => Some(Line::new_data(i, parts[0].to_string(), data)),
        Err(msg) => Some(Line::new_error(i, Expected::Data, msg)),
    }
}

pub fn parse_data_value(str: &str) -> Result<Vec<u8>, String> {
    let str = str.trim();
    if str.starts_with('"') && str.ends_with('"') {
        let text = str.trim_matches('"');
        return if !text
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || SYMBOLS.contains(&c))
        {
            Err("Unsupported characters in string".to_string())
        } else {
            Ok(text.chars().map(|c| c as u8).collect())
        };
    } else if str.contains(',') {
        let mut bytes = vec![];
        for num_str in str.split(',') {
            match u8::from_str(num_str) {
                Ok(num) => bytes.push(num),
                Err(err) => return Err(format!("Unable to data numbers: {err}")),
            }
        }
        Ok(bytes)
    } else {
        if !str.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Invalid hex digit".to_string());
        }
        let list = str.chars().collect::<Vec<char>>();
        if list.len() % 2 != 0 {
            return Err("Invalid hex data, must be pairs of two characters".to_string());
        }
        let bytes = list
            .chunks_exact(2)
            .map(|chrs| {
                u8::from_str_radix(&format!("{}{}", chrs[0], chrs[1]), 16)
                    .expect("Parse failed after checking validity, please raise an issue")
            })
            .collect();
        Ok(bytes)
    }
}

#[cfg(test)]
mod test {
    use crate::parser::line::tokens::Condition::Eq;
    use crate::parser::line::tokens::Param::*;
    use crate::parser::line::tokens::Token::*;
    use super::*;

    #[test]
    fn check_parse_basic() {
        assert_eq!(parse_code(0, "clear", &[]), Some(Line::Code {
            line: 0,
            label: None,
            token: Clear,
        }));
        assert_eq!(parse_code(1, "lbl: v0 += 3", &[]), Some(Line::Code {
            line: 1,
            label: Some("lbl".to_string()),
            token: Add(Reg(0), Num(3)),
        }));
        assert_eq!(parse_code(4, "if eq(v3,va) break", &[]), Some(Line::Code {
            line: 4,
            label: None,
            token: If(Eq(false, Reg(3), Reg(10)),Box::new(Break))
        }));
    }
}