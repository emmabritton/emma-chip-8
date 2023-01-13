use crate::parser::line::tokens::Param::*;
use crate::parser::line::tokens::{Param, Token};

impl Token {
    pub fn validate(&self) -> Option<String> {
        match self {
            Token::Loop => None,
            Token::Again => None,
            Token::Break => None,
            Token::Return => None,
            Token::Clear => None,
            Token::MacroCall(_, _) => None,
            Token::Add(p1, p2) => validate_add(p1, p2),
            Token::Sub(p1, p2) => validate_rr("Sub", p1, p2),
            Token::Subr(p1, p2) => validate_rr("Subr", p1, p2),
            Token::Or(p1, p2) => validate_rr("Or", p1, p2),
            Token::Xor(p1, p2) => validate_rr("Xor", p1, p2),
            Token::And(p1, p2) => validate_rr("And", p1, p2),
            Token::Set(p1, p2) => validate_set(p1, p2),
            Token::Shr(p1) => validate_r("Shr", p1),
            Token::Shl(p1) => validate_r("Shl", p1),
            Token::WaitForKey(p1) => validate_r("Wait for key", p1),
            Token::Rand(p1, p2) => validate_rn("Rand", p1, p2),
            Token::Draw(p1, p2, p3) => validate_rrn("Draw", p1, p2, p3),
            Token::StoreReg(p1) => validate_r("Store regs", p1),
            Token::LoadReg(p1) => validate_r("Load regs", p1),
            Token::Bcd(p1) => validate_r("BCD", p1),
            Token::If(_, if_token) => if_token.validate(),
            Token::Goto(p1) => validate_a("Call", p1),
            Token::GotoOffset(p1, p2) => validate_ar("Goto offset", p1, p2),
            Token::Digit(p1) => validate_r("Digit", p1),
            Token::Ascii(p1) => validate_r("Ascii", p1),
            Token::Call(p1) => validate_a("Call", p1),
            Token::MacroStart(_, _) => None,
            Token::MacroEnd => None,
        }
    }
}

fn validate_ar(op: &str, p1: &Param, p2: &Param) -> Option<String> {
    match (p1, p2) {
        (Addr(_) | Label(_) | Data(_) | Unknown(_), Reg(_)) => None,
        _ => Some(format!("{op} only supports A,R  L,R  D,R")),
    }
}

fn validate_a(op: &str, p1: &Param) -> Option<String> {
    match p1 {
        Addr(_) | Label(_) | Unknown(_) | Data(_) => None,
        _ => Some(format!("{op} only supports A  L")),
    }
}

fn validate_r(op: &str, p1: &Param) -> Option<String> {
    match p1 {
        Reg(_) => None,
        _ => Some(format!("{op} only supports R")),
    }
}

fn validate_rn(op: &str, p1: &Param, p2: &Param) -> Option<String> {
    match (p1, p2) {
        (Reg(_), Num(_)) => None,
        (_, _) => Some(format!("{op} only supports R,N")),
    }
}

fn validate_rrn(op: &str, p1: &Param, p2: &Param, p3: &Param) -> Option<String> {
    match (p1, p2, p3) {
        (Reg(_), Reg(_), Num(_)) => None,
        (_, _, _) => Some(format!("{op} only supports R,R,N")),
    }
}

fn validate_rr(op: &str, p1: &Param, p2: &Param) -> Option<String> {
    match (p1, p2) {
        (Reg(_), Reg(_)) => None,
        (_, _) => Some(format!("{op} only supports R,R")),
    }
}

fn validate_set(p1: &Param, p2: &Param) -> Option<String> {
    match (p1, p2) {
        (Reg(_), Reg(_)) => None,
        (Reg(_), Num(_)) => None,
        (MemReg, Label(_)) => None,
        (MemReg, Addr(_)) => None,
        (MemReg, Data(_)) => None,
        (_, _) => Some("Assign only supports R,R  R,N  I,L  I,A  I,D".to_string()),
    }
}

fn validate_add(p1: &Param, p2: &Param) -> Option<String> {
    match (p1, p2) {
        (Reg(_), Reg(_)) => None,
        (Reg(_), Num(_)) => None,
        (MemReg, Reg(_)) => None,
        (_, _) => Some("Add only supports R,R  R,N  I,R".to_string()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_validate() {
        assert!(Token::WaitForKey(Reg(0)).validate().is_none());
        assert!(Token::WaitForKey(Addr(0)).validate().is_some());
        assert!(Token::WaitForKey(Data(String::new())).validate().is_some());
    }

    #[test]
    fn check_validate_ar() {
        assert!(validate_ar("", &Addr(0), &Reg(0)).is_none());
        assert!(validate_ar("", &Label(String::new()), &Reg(0)).is_none());
        assert!(validate_ar("", &Data(String::new()), &Reg(0)).is_none());
        assert!(validate_ar("", &Unknown(String::new()), &Reg(0)).is_none());
        assert!(validate_ar("", &Label(String::new()), &Reg(0)).is_none());
        assert!(validate_ar("", &Reg(0), &Num(0)).is_some());
        assert!(validate_ar("", &MemReg, &Reg(0)).is_some());
    }

    #[test]
    fn check_validate_a() {
        assert!(validate_a("", &Addr(0)).is_none());
        assert!(validate_a("", &Data(String::new())).is_none());
        assert!(validate_a("", &Unknown(String::new())).is_none());
        assert!(validate_a("", &Label(String::new())).is_none());
        assert!(validate_a("", &Reg(0)).is_some());
        assert!(validate_a("", &Num(0)).is_some());
        assert!(validate_a("", &MemReg).is_some());
    }

    #[test]
    fn check_validate_r() {
        assert!(validate_r("", &Reg(0)).is_none());
        assert!(validate_r("", &Num(0)).is_some());
        assert!(validate_r("", &MemReg).is_some());
    }

    #[test]
    fn check_validate_rn() {
        assert!(validate_rn("", &Reg(0), &Reg(0)).is_some());
        assert!(validate_rn("", &Reg(0), &Num(0)).is_none());
        assert!(validate_rn("", &MemReg, &Reg(0)).is_some());
    }

    #[test]
    fn check_validate_rrn() {
        assert!(validate_rrn("", &Reg(0), &Reg(0), &Num(0)).is_none());
        assert!(validate_rrn("", &Reg(0), &Num(0), &Reg(0)).is_some());
        assert!(validate_rrn("", &MemReg, &Reg(0), &MemReg).is_some());
    }

    #[test]
    fn check_validate_rr() {
        assert!(validate_rr("", &Reg(0), &Reg(0)).is_none());
        assert!(validate_rr("", &Reg(0), &Num(0)).is_some());
        assert!(validate_rr("", &MemReg, &Reg(0)).is_some());
    }

    #[test]
    fn check_validate_set() {
        assert!(validate_set(&Reg(0), &Reg(0)).is_none());
        assert!(validate_set(&Reg(0), &Num(0)).is_none());
        assert!(validate_set(&MemReg, &Addr(0)).is_none());
        assert!(validate_set(&MemReg, &Label(String::new())).is_none());
        assert!(validate_set(&MemReg, &Data(String::new())).is_none());
        assert!(validate_set(&MemReg, &Num(0)).is_some());
        assert!(validate_set(&Reg(0), &MemReg).is_some());
        assert!(validate_set(&Reg(0), &Delay).is_some());
    }

    #[test]
    fn check_validate_add() {
        assert!(validate_add(&Reg(0), &Reg(0)).is_none());
        assert!(validate_add(&Reg(0), &Num(0)).is_none());
        assert!(validate_add(&MemReg, &Reg(0)).is_none());
        assert!(validate_add(&MemReg, &Num(0)).is_some());
        assert!(validate_add(&Reg(0), &MemReg).is_some());
        assert!(validate_add(&Reg(0), &Delay).is_some());
    }
}
