use crate::OpCodes;

impl OpCodes {
    pub fn mnemonic(&self) -> &'static str {
        match self {
            OpCodes::SysCall => "",
            OpCodes::ClearDisplay => "clr",
            OpCodes::Return => "ret",
            OpCodes::Jump => "jmp",
            OpCodes::Call => "cal",
            OpCodes::SkipIfEqualNum => "ske",
            OpCodes::SkipIfNotEqualNum => "skn",
            OpCodes::SkipIfEqualReg => "ske",
            OpCodes::SetRegFromNum => "set",
            OpCodes::AddNumToReg => "add",
            OpCodes::SetRegFromReg => "set",
            OpCodes::BitwiseOr => "or",
            OpCodes::BitwiseAnd => "and",
            OpCodes::BitwiseXor => "xor",
            OpCodes::AddReg => "add",
            OpCodes::SubRightReg => "sub",
            OpCodes::ShiftRight => "shr",
            OpCodes::SubLeftReg => "sbr",
            OpCodes::ShiftLeft => "shl",
            OpCodes::SkipIfNotEqualReg => "skn",
            OpCodes::SetMemReg => "sti",
            OpCodes::JumpOffset => "jpo",
            OpCodes::SetRegRand => "rnd",
            OpCodes::DrawSprite => "drw",
            OpCodes::SkipIfKeyPressed => "skp",
            OpCodes::SkipIfKeyNotPressed => "skr",
            OpCodes::SetRegFromTimer => "rdt",
            OpCodes::WaitForKey => "key",
            OpCodes::SetDelayTimer => "sdt",
            OpCodes::SetSoundTimer => "sst",
            OpCodes::AddMemReg => "adi",
            OpCodes::SetMemRegToDigitSprite => "chr",
            OpCodes::SetMemRegToAsciiSprite => "asc",
            OpCodes::StoreBcd => "bcd",
            OpCodes::StoreRegs => "str",
            OpCodes::LoadRegs => "ldr"
        }
    }
}