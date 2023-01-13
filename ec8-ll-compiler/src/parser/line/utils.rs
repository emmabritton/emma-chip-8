use crate::parser::line::Definition;

pub const SYMBOLS: [char; 31] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '=', '_', '+', '{', '}', '[', ']', ';',
    ':', '"', '|', ',', '.', '/', '<', '>', '?', '~', '\'', '\\',
];
const DATA_REGISTERS: [&str; 23] = [
    "v0", "v1", "v2", "v3", "v4", "v5", "v6", "v7", "v8", "v9", "va", "vb", "vc", "vd", "ve", "vf",
    "v10", "v11", "v12", "v13", "v14", "v15", "flag",
];
const OTHER_REGISTERS: [&str; 4] = ["i", "mreg", "delay", "sound"];
const ADDRESSES: [&str; 4] = ["prog", "g_digit", "g_alpha", "g_sym"];
const KEYWORDS: [&str; 21] = [
    "if",
    "alias",
    "loop",
    "data",
    "break",
    "again",
    "shr",
    "shl",
    "rand",
    "draw",
    "digit",
    "ascii",
    "goto",
    "call",
    "return",
    "clear",
    "jump",
    "bcd",
    "wait_for_key",
    "reg_store",
    "reg_load",
];
const MACROS: [&str; 3] = ["draw_digit", "draw_ascii", "read_data"];
const CONDITIONALS: [&str; 2] = ["pressed", "eq"];

const BUILTINS: [(&str, &[&str]); 6] = [
    ("data reg", &DATA_REGISTERS),
    ("register", &OTHER_REGISTERS),
    ("keyword", &KEYWORDS),
    ("macro", &MACROS),
    ("address", &ADDRESSES),
    ("conditional", &CONDITIONALS),
];

pub fn check_name(name: &str, defs: &[Definition]) -> Option<String> {
    let name = name.to_lowercase().trim().to_string();
    if !name.is_ascii() {
        return Some(format!("'{name}' must be ASCII"));
    }
    if !name
        .chars()
        .all(|c| char::is_ascii_alphanumeric(&c) || c == '_')
    {
        return Some(format!(
            "'{name}' must only contain ASCII letters, numbers and underscore"
        ));
    }
    if let Some(err) = check_all_builtins(&name) {
        return Some(format!("'{name}' clashes with {err}"));
    }
    for def in defs {
        if def.name == name {
            return Some(format!(
                "'{name}' already defined as {:?} on line {}",
                def.def_type, def.line
            ));
        }
    }
    None
}

fn check_all_builtins(user_def: &str) -> Option<String> {
    for (name, list) in BUILTINS {
        if list.contains(&user_def) {
            return Some(name.to_string());
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_builtins() {
        assert!(check_all_builtins("def_not_in").is_none());
        assert_eq!(check_all_builtins("break"), Some("keyword".to_string()));
        assert_eq!(check_all_builtins("v0"), Some("data reg".to_string()));
    }
}
