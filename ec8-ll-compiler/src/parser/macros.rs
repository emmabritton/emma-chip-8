use crate::parser::line::tokens::Token::*;
use crate::parser::line::tokens::{MacroParam, Param, Token};
use crate::parser::line::Line;
use std::collections::HashMap;
use crate::parser::line::tokens::Param::{MemReg, Num, Placeholder, Reg};

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    params: Vec<MacroParam>,
    tokens: Vec<Token>,
}

impl Macro {
    pub fn new(name: String, params: Vec<MacroParam>) -> Self {
        Self {
            name,
            params,
            tokens: vec![],
        }
    }

    pub fn builtin_macros() -> Vec<Macro> {
        vec![
            Macro {
                name: "draw_ascii".to_string(),
                params: vec![MacroParam::Reg, MacroParam::Reg, MacroParam::Reg],
                tokens: vec![
                    Ascii(Placeholder(3)),
                    Draw(Placeholder(1), Placeholder(2), Num(5))
                ]
            },
            Macro {
                name: "draw_digit".to_string(),
                params: vec![MacroParam::Reg, MacroParam::Reg, MacroParam::Reg],
                tokens: vec![
                    Digit(Placeholder(3)),
                    Draw(Placeholder(1), Placeholder(2), Num(5))
                ]
            },
            Macro {
                name: "read_data".to_string(),
                params: vec![MacroParam::DataAddr, MacroParam::Reg],
                tokens: vec![
                    Set(MemReg, Placeholder(1)),
                    Add(MemReg, Placeholder(2)),
                    LoadReg(Reg(0))
                ]
            }
        ]
    }
}

impl Macro {
    pub fn expand(&self, line: usize, params: &[Param]) -> Result<Vec<Line>, String> {
        if params.len() != params.len() {
            return Err(format!(
                "Call to macro {} on line {line} is invalid as macro requires ({}) params",
                self.name,
                self.params
                    .iter()
                    .map(|param| format!("{:?}", param))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }
        for (i, macro_param) in self.params.iter().enumerate() {
            macro_param.check_compat(line, i, &params[i])?;
        }
        Ok(self
            .tokens
            .iter()
            .map(|token| token.replace_placeholders(params))
            .map(|token| Line::new_code(line, None, token))
            .collect())
    }
}

#[allow(clippy::type_complexity)]
pub fn extract_macros(lines: Vec<Line>) -> Result<(Vec<Line>, HashMap<String, Macro>), String> {
    let mut output = vec![];
    let mut macros = HashMap::new();
    let mut current_macro: Option<(usize, Macro)> = None;
    for (i, line) in lines.into_iter().enumerate() {
        if let Line::Code {
            line: _,
            label: _,
            token,
        } = &line
        {
            if let Some((start, macro_def)) = &mut current_macro {
                check_line_allowed(i, &line, &Some((*start, macro_def.clone())))?;
                if let MacroStart(name, _) = token {
                    return Err(format!("Can not nest macros, macro {} started at {start} and nested macro {name} started at {i}", macro_def.name));
                } else if let MacroEnd = token {
                    macros.insert(macro_def.name.clone(), macro_def.clone());
                    current_macro = None;
                } else {
                    macro_def.tokens.push(token.clone());
                }
            } else if let MacroStart(name, params) = token {
                current_macro = Some((i, Macro::new(name.clone(), params.clone())))
            } else if let MacroEnd = token {
                return Err(format!("Found marco end on line {i} but no macro started"));
            } else {
                output.push(line.clone());
            }
        } else {
            output.push(line.clone());
        }
    }

    Ok((output, macros))
}

pub fn expand_macros(
    lines: Vec<Line>,
    macros: HashMap<String, Macro>,
) -> Result<Vec<Line>, String> {
    let mut output = vec![];

    for line in lines {
        if let Line::Code {
            line: i,
            label: _,
            token,
        } = &line
        {
            if let MacroCall(name, params) = token {
                match macros.get(name) {
                    None => return Err(format!("Call to undefined macro {name} on line {i}")),
                    Some(macro_def) => {
                        let tokens = macro_def.expand(*i, params)?;
                        output.extend_from_slice(&tokens);
                    }
                }
            } else {
                output.push(line.clone());
            }
        } else {
            output.push(line);
        }
    }

    Ok(output)
}

fn check_line_allowed(
    i: usize,
    line: &Line,
    current_macro: &Option<(usize, Macro)>,
) -> Result<(), String> {
    match line {
        Line::Alias { .. } => {
            if let Some((start, macro_def)) = current_macro {
                return Err(format!("Alias is not allowed in macros, on line {i} inside macro {} started at line {start}", macro_def.name));
            }
        }
        Line::Data { .. } => {
            if let Some((start, macro_def)) = current_macro {
                return Err(format!("Data is not allowed in macros, on line {i} inside macro {} started at line {start}", macro_def.name));
            }
        }
        Line::Code {
            line: _,
            label,
            token,
        } => {
            if let Some((start, macro_def)) = current_macro {
                if label.is_some() {
                    return Err(format!("Labels are not allowed in macros, on line {i} inside macro {} started at line {start}", macro_def.name));
                }
                if matches!(token, Loop | Again | Break | MacroStart(_, _)) {
                    return Err(format!("{:?} is not allowed in macros, on line {i} inside macro {} started at line {start}", token, macro_def.name));
                }
            }
        }
        Line::Label { .. } => {
            if let Some((start, macro_def)) = current_macro {
                return Err(format!("Labels are not allowed in macros, on line {i} inside macro {} started at line {start}", macro_def.name));
            }
        }
        Line::Error {
            line: _,
            expected: _,
            message,
        } => panic!("Found error '{message}' when extracting macros, please raise an issue"),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::parser::line::tokens::MacroParam;
    use crate::parser::line::tokens::Param::{Placeholder, Reg};
    use crate::parser::line::tokens::Token::{Clear, Digit};
    use crate::parser::line::Line;
    use crate::parser::macros::Macro;

    #[test]
    fn check_expand() {
        let mut test_macro = Macro::new("tst".to_string(), vec![MacroParam::Reg]);
        test_macro.tokens = vec![Clear, Digit(Placeholder(1))];
        let lines = test_macro.expand(2, &[Reg(3)]).unwrap();
        assert_eq!(
            lines,
            &[
                Line::new_code(2, None, Clear),
                Line::new_code(2, None, Digit(Reg(3))),
            ]
        );
    }
}
