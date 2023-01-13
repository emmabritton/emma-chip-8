use crate::ast::{AsmLine, Program};
use crate::parser::line::tokens::Param;
use ec8_common::OpCodes;
use crate::ast::data::Data;

impl Program {
    pub fn to_asm(&self) -> Vec<String> {
        let mut output = vec![];
        for line in &self.asm_lines {
            output.push(line.to_asm())
        }
        for data in &self.datas {
            output.push(data.to_asm())
        }
        output
    }
}

impl Data {
    pub fn to_asm(&self) -> String {
        format!("dat [{}]", self.bytes.iter().map(|byt| format!("{:02X}", byt)).collect::<Vec<String>>().join(""))
    }
}

impl AsmLine {
    pub fn to_asm(&self) -> String {
        match self.opcode {
            OpCodes::SysCall | OpCodes::Return | OpCodes::ClearDisplay => {
                self.opcode.mnemonic().to_string()
            }
            OpCodes::SkipIfNotEqualReg
            | OpCodes::SkipIfEqualReg
            | OpCodes::SkipIfNotEqualNum
            | OpCodes::SkipIfEqualNum
            | OpCodes::SetRegRand
            | OpCodes::SetRegFromNum
            | OpCodes::AddNumToReg
            | OpCodes::SetRegFromReg
            | OpCodes::AddReg
            | OpCodes::SubLeftReg
            | OpCodes::SubRightReg
            | OpCodes::BitwiseAnd
            | OpCodes::BitwiseXor
            | OpCodes::BitwiseOr => format!(
                "{} {}, {}",
                self.opcode.mnemonic(),
                self.params[0].to_asm(),
                self.params[1].to_asm()
            ),
            OpCodes::DrawSprite => format!(
                "{} {}, {}, {}",
                self.opcode.mnemonic(),
                self.params[0].to_asm(),
                self.params[1].to_asm(),
                self.params[2].to_asm()
            ),
            OpCodes::SkipIfKeyPressed
            | OpCodes::SkipIfKeyNotPressed
            | OpCodes::SetRegFromTimer
            | OpCodes::SetDelayTimer
            | OpCodes::SetSoundTimer
            | OpCodes::AddMemReg
            | OpCodes::JumpOffset
            | OpCodes::SetMemReg
            | OpCodes::Jump
            | OpCodes::Call
            | OpCodes::SetMemRegToDigitSprite
            | OpCodes::SetMemRegToAsciiSprite
            | OpCodes::ShiftRight
            | OpCodes::ShiftLeft
            | OpCodes::WaitForKey
            | OpCodes::StoreBcd
            | OpCodes::StoreRegs
            | OpCodes::LoadRegs => {
                format!("{} {}", self.opcode.mnemonic(), self.params[0].to_asm())
            }
        }
    }
}

impl Param {
    pub fn to_asm(&self) -> String {
        match self {
            Param::Reg(n) => format!("v{n}"),
            Param::Addr(nnn) => format!("{:03X}", nnn),
            Param::Num(n) => format!("{:02X}", n),
            Param::MemReg => "I".to_string(),
            Param::Data(txt) => txt.to_string(),
            _ => panic!("Encountered {:?} when converting to asm", self)
        }
    }
}
