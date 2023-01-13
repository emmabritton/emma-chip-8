use crate::parser::line::models::{Definition, DefinitionType};
use crate::parser::line::tokens::Param;
use std::str::FromStr;

pub fn cleanup(parts: &[&str]) -> Vec<String> {
    parts
        .iter()
        .map(|part| {
            part.trim()
                .trim_start_matches(|c| matches!(c, '(' | ','))
                .trim_end_matches(|c| matches!(c, ')' | ','))
        })
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect()
}

pub fn parse_param(param: &str, defs: &[Definition]) -> Result<Param, String> {
    let param = param
        .trim()
        .to_lowercase()
        .trim_end_matches(')')
        .to_string();
    for def in defs {
        if param == def.name {
            return match def.def_type {
                DefinitionType::Label => Ok(Param::Label(param)),
                DefinitionType::Alias => parse_param(&def.value, defs),
                DefinitionType::Data => Ok(Param::Data(param)),
            };
        }
    }
    if param == "sound" {
        return Ok(Param::Sound);
    }
    if param == "delay" {
        return Ok(Param::Delay);
    }
    if param == "i" {
        return Ok(Param::MemReg);
    }
    if param.starts_with('$') {
        let i = u8::from_str(param.trim_start_matches('$')).map_err(|err| err.to_string())?;
        return if i > 0 {
            Ok(Param::Placeholder(i))
        } else {
            Err("Placeholders start at 1".to_string())
        };
    }
    if param.starts_with('v') {
        let num = param.trim_start_matches('v');
        let num = if num.chars().count() == 2 {
            u8::from_str(num)
        } else {
            u8::from_str_radix(num, 16)
        }
        .map_err(|err| err.to_string())?;
        return Ok(Param::Reg(num));
    }
    if param.starts_with('\'') && param.ends_with('\'') && param.len() == 3 {
        let chr = param.trim_matches('\'');
        return Ok(Param::Num(chr.chars().next().unwrap() as u8));
    }
    if param.starts_with('-') {
        return match i8::from_str(&param) {
            Ok(num) => Ok(Param::Num(num as u8)),
            Err(err) => Err(format!("Can't parse param {param}: {err}")),
        };
    }
    if param.starts_with('x') {
        return match u8::from_str_radix(param.trim_start_matches('x'), 16) {
            Ok(num) => Ok(Param::Num(num)),
            Err(err) => Err(format!("Can't parse param {param}: {err}")),
        };
    }
    if param.starts_with('b') {
        return match u8::from_str_radix(param.trim_start_matches('b'), 2) {
            Ok(num) => Ok(Param::Num(num)),
            Err(err) => Err(format!("Can't parse param {param}: {err}")),
        };
    }
    if param.starts_with('@') {
        return if param.starts_with("@x") {
            match u16::from_str_radix(param.trim_start_matches("@x"), 16) {
                Ok(num) => Ok(Param::Addr(num)),
                Err(err) => Err(format!("Can't parse param {param}: {err}")),
            }
        } else {
            match u16::from_str(param.trim_start_matches('@')) {
                Ok(num) => Ok(Param::Addr(num)),
                Err(err) => Err(format!("Can't parse param {param}: {err}")),
            }
        };
    }
    if let Ok(num) = u8::from_str(&param) {
        return Ok(Param::Num(num));
    }
    Ok(Param::Unknown(param.clone()))
}
