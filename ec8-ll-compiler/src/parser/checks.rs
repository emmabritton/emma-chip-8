use crate::parser::line::tokens::Param::MemReg;
use crate::parser::line::tokens::{Param, Token};
use crate::parser::line::Line;

pub fn verify_labels(lines: &[Line]) -> Result<Option<String>, String> {
    let mut errors = vec![];
    let mut used_labels = vec![];
    let mut used_datas = vec![];

    let mut labels = vec![];
    let mut datas = vec![];
    for line in lines {
        match line {
            Line::Alias { .. } => {}
            Line::Data {
                line,
                name,
                data: _,
            } => {
                if datas.contains(&name) || labels.contains(&name) {
                    errors.push(format!("Data {name} on line {line} already defined"));
                }
                datas.push(name);
            }
            Line::Code { line, label, token } => {
                if let Some(label) = label {
                    if labels.contains(&label) || datas.contains(&label) {
                        errors.push(format!("Label {label} on line {line} already defined"));
                    }
                }
                match token {
                    Token::Set(target, value) => {
                        if target == &MemReg {
                            match value {
                                Param::Label(lbl) => used_labels.push(lbl),
                                Param::Data(dat) => used_datas.push(dat),
                                _ => {}
                            }
                        }
                    }
                    Token::GotoOffset(dest, _) | Token::Call(dest) | Token::Goto(dest) => {
                        match dest {
                            Param::Label(lbl) => used_labels.push(lbl),
                            Param::Data(dat) => used_datas.push(dat),
                            _ => {}
                        }
                    }
                    Token::MacroCall(_, params) => {
                        for param in params {
                            match param {
                                Param::Label(lbl) => used_labels.push(lbl),
                                Param::Data(dat) => used_datas.push(dat),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            Line::Label { line, name } => {
                if labels.contains(&name) || datas.contains(&name) {
                    errors.push(format!("Label {name} on line {line} already defined"));
                }
                labels.push(name);
            }
            Line::Error { .. } => {}
        }
    }

    let unused_labels = labels
        .iter()
        .filter(|lbl| !used_labels.contains(lbl))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let unused_datas = datas
        .iter()
        .filter(|dat| !used_datas.contains(dat))
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut warning = String::new();
    if !unused_labels.is_empty() {
        warning.push_str(&format!(
            "These labels were never used: {}",
            unused_labels.join(", ")
        ));
    }
    if !unused_datas.is_empty() {
        if !warning.is_empty() {
            warning.push('\n');
        }
        warning.push_str(&format!(
            "These datas were never used: {}",
            unused_datas.join(", ")
        ));
    }

    let mut text = String::new();
    if !warning.is_empty() {
        text.push_str("Warnings:\n");
        text.push_str(&warning);
    }
    if !errors.is_empty() {
        text.push_str("Errors:\n");
        text.push_str(&errors.join("\n"));
    }

    if !text.is_empty() {
        Err(text)
    } else if !warning.is_empty() {
        Ok(Some(warning))
    } else {
        Ok(None)
    }
}

pub fn validate_code(lines: &[Line]) -> Result<(), String> {
    let mut output = String::new();

    for line in lines {
        if let Line::Code {
            line,
            label: _,
            token,
        } = line
        {
            if let Some(text) = token.validate() {
                output.push_str(&format!("\nLine {line}: {text}"))
            }
        }
    }

    if !output.is_empty() {
        Err(format!("Syntax error:\n{output}"))
    } else {
        Ok(())
    }
}

pub fn handle_errors(lines: &[Line]) -> Result<(), String> {
    let mut output = String::new();

    for line in lines {
        if let Line::Error {
            line,
            expected,
            message,
        } = line
        {
            output.push_str(&format!(
                "\nLine {line}: expected {:?} line. Error: {message}",
                expected
            ));
        }
    }

    if !output.is_empty() {
        Err(format!("Unable to compile:\n{output}"))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::parser::checks::{handle_errors, verify_labels};
    use crate::parser::line::tokens::Param::Label;
    use crate::parser::line::tokens::Token;
    use crate::parser::line::tokens::Token::{Break, Goto};
    use crate::parser::line::{Expected, Line};

    #[test]
    fn check_labels_that_does_exist() {
        let lines = vec![
            Line::new_label(0, "lbl".to_string()),
            Line::new_code(1, None, Goto(Label("lbl".to_string()))),
        ];
        assert!(verify_labels(&lines).is_ok());
    }

    #[test]
    fn check_labels_that_exists_twice() {
        let lines = vec![
            Line::new_label(0, "lbl".to_string()),
            Line::new_code(1, Some("lbl".to_string()), Break),
        ];
        assert!(verify_labels(&lines).is_err());
    }

    #[test]
    fn check_handle_errors() {
        assert!(handle_errors(&[Line::new_error(0, Expected::Alias, String::new())]).is_err());
        assert!(handle_errors(&[Line::new_code(0, None, Token::Again)]).is_ok());
    }
}
