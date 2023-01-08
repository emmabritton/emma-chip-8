use crate::error::ECommonError::*;
use crate::error::ECommonResult;
use crate::nibbler::Nibbler;
use crate::OpCodes;
use crate::OpCodes::*;

pub fn from_bytes(bytes: [u8; 2]) -> Option<OpCodes> {
    let first_nibble = bytes[0].first_nibble();
    let last_nibble = bytes[1].second_nibble();
    match first_nibble {
        0x00 => match bytes[1] {
            0xE0 => Some(ClearDisplay),
            0xEE => Some(Return),
            _ => Some(SysCall),
        },
        0x10 => Some(Jump),
        0x20 => Some(Call),
        0x30 => Some(SkipIfEqualNum),
        0x40 => Some(SkipIfNotEqualNum),
        0x50 => Some(SkipIfEqualReg),
        0x60 => Some(SetRegFromNum),
        0x70 => Some(AddNumToReg),
        0x80 => match last_nibble {
            0x0 => Some(SetRegFromReg),
            0x1 => Some(BitwiseOr),
            0x2 => Some(BitwiseAnd),
            0x3 => Some(BitwiseXor),
            0x4 => Some(AddReg),
            0x5 => Some(SubRightReg),
            0x6 => Some(ShiftRight),
            0x7 => Some(SubLeftReg),
            0xE => Some(ShiftLeft),
            _ => None,
        },
        0x90 => Some(SkipIfNotEqualReg),
        0xA0 => Some(SetMemReg),
        0xB0 => Some(JumpOffset),
        0xC0 => Some(SetRegRand),
        0xD0 => Some(DrawSprite),
        0xE0 => match bytes[1] {
            0x9E => Some(SkipIfKeyPressed),
            0xA1 => Some(SkipIfKeyNotPressed),
            _ => None,
        },
        0xF0 => match bytes[1] {
            0x07 => Some(SetRegFromTimer),
            0x0A => Some(WaitForKey),
            0x15 => Some(SetDelayTimer),
            0x18 => Some(SetSoundTimer),
            0x1E => Some(AddMemReg),
            0x29 => Some(SetMemRegToDigitSprite),
            0x33 => Some(StoreBcd),
            0x55 => Some(StoreRegs),
            0x65 => Some(LoadRegs),
            _ => None,
        },
        _ => None,
    }
}

pub fn no_param(opcode: OpCodes) -> ECommonResult<u16> {
    match opcode {
        ClearDisplay => Ok(0x00E0),
        Return => Ok(0x00EE),
        _ => Err(InvalidOpCode(opcode)),
    }
}

pub fn reg(opcode: OpCodes, reg: u8) -> ECommonResult<u16> {
    let x = cast_first_reg(reg);
    match opcode {
        SkipIfKeyPressed => Ok(0xE09E | x),
        SkipIfKeyNotPressed => Ok(0xE0A1 | x),
        SetRegFromTimer => Ok(0xF007 | x),
        WaitForKey => Ok(0xF00A | x),
        SetDelayTimer => Ok(0xF015 | x),
        SetSoundTimer => Ok(0xF018 | x),
        AddMemReg => Ok(0xF01E | x),
        SetMemRegToDigitSprite => Ok(0xF029 | x),
        StoreBcd => Ok(0xF033 | x),
        StoreRegs => Ok(0xF055 | x),
        LoadRegs => Ok(0xF065 | x),
        _ => Err(InvalidOpCode(opcode)),
    }
}

pub fn reg_reg_num(opcode: OpCodes, reg_x: u8, reg_y: u8, num: u8) -> ECommonResult<u16> {
    if num > 0xF {
        return Err(NumberTooLarge(num));
    }
    reg_reg_num_unchecked(opcode, reg_x, reg_y, num)
}

pub fn reg_reg_num_unchecked(opcode: OpCodes, reg_x: u8, reg_y: u8, num: u8) -> ECommonResult<u16> {
    let x = cast_first_reg(reg_x);
    let y = cast_second_reg(reg_y);
    let n = cast_4bit_num(num);
    match opcode {
        DrawSprite => Ok(((0xD000 | x) | y) | n),
        _ => Err(InvalidOpCode(opcode)),
    }
}

pub fn reg_reg(opcode: OpCodes, reg_x: u8, reg_y: u8) -> ECommonResult<u16> {
    let x = cast_first_reg(reg_x);
    let y = cast_second_reg(reg_y);
    match opcode {
        SkipIfEqualReg => Ok((0x5000 | x) | y),
        SetRegFromReg => Ok((0x8000 | x) | y),
        BitwiseOr => Ok((0x8001 | x) | y),
        BitwiseAnd => Ok((0x8002 | x) | y),
        BitwiseXor => Ok((0x8003 | x) | y),
        AddReg => Ok((0x8004 | x) | y),
        SubRightReg => Ok((0x8005 | x) | y),
        ShiftRight => Ok((0x8006 | x) | y),
        SubLeftReg => Ok((0x8007 | x) | y),
        ShiftLeft => Ok((0x800E | x) | y),
        SkipIfNotEqualReg => Ok((0x9000 | x) | y),
        _ => Err(InvalidOpCode(opcode)),
    }
}

pub fn reg_num(opcode: OpCodes, reg_x: u8, num: u8) -> ECommonResult<u16> {
    let x = cast_first_reg(reg_x);
    let n = cast_8bit_num(num);
    match opcode {
        SkipIfEqualNum => Ok((0x3000 | x) | n),
        SkipIfNotEqualNum => Ok((0x4000 | x) | n),
        SetRegFromNum => Ok((0x6000 | x) | n),
        AddNumToReg => Ok((0x7000 | x) | n),
        SetRegRand => Ok((0xC000 | x) | n),
        _ => Err(InvalidOpCode(opcode)),
    }
}

pub fn address(opcode: OpCodes, address: u16) -> ECommonResult<u16> {
    if address > 0xFFF {
        return Err(AddressOutOfRange(address));
    }
    address_unchecked(opcode, address)
}

pub fn address_unchecked(opcode: OpCodes, address: u16) -> ECommonResult<u16> {
    let addr = address & 0xFFF;
    match opcode {
        SysCall => Ok(addr),
        Jump => Ok(0x1000 | addr),
        Call => Ok(0x2000 | addr),
        SetMemReg => Ok(0xA000 | addr),
        JumpOffset => Ok(0xB000 | addr),
        _ => Err(InvalidOpCode(opcode)),
    }
}

fn cast_4bit_num(num: u8) -> u16 {
    (num as u16) & 0x000F
}

fn cast_8bit_num(num: u8) -> u16 {
    (num as u16) & 0x00FF
}

fn cast_first_reg(num: u8) -> u16 {
    ((num as u16) << 8) & 0x0F00
}

fn cast_second_reg(num: u8) -> u16 {
    ((num as u16) << 4) & 0x00F0
}

#[cfg(test)]
mod test {
    use crate::opcodes::*;

    #[test]
    fn check_from_byte() {
        assert_eq!(from_bytes([0xF1, 0x07]), Some(SetRegFromTimer));
        assert_eq!(from_bytes([0xFF, 0x07]), Some(SetRegFromTimer));
        assert_eq!(from_bytes([0x8F, 0x07]), Some(SubLeftReg));
    }

    #[test]
    fn check_num_casts() {
        assert_eq!(cast_4bit_num(0), 0);
        assert_eq!(cast_4bit_num(1), 1);
        assert_eq!(cast_4bit_num(8), 8);
        assert_eq!(cast_4bit_num(15), 15);
        assert_eq!(cast_4bit_num(16), 0);

        assert_eq!(cast_8bit_num(0), 0);
        assert_eq!(cast_8bit_num(1), 1);
        assert_eq!(cast_8bit_num(128), 128);
        assert_eq!(cast_8bit_num(255), 255);
    }

    #[test]
    fn check_reg_casts() {
        assert_eq!(cast_first_reg(0), 0x0000);
        assert_eq!(cast_first_reg(1), 0x0100);
        assert_eq!(cast_first_reg(15), 0x0F00);

        assert_eq!(cast_second_reg(0), 0x0000);
        assert_eq!(cast_second_reg(1), 0x0010);
        assert_eq!(cast_second_reg(15), 0x00F0);
    }

    #[test]
    fn check_two_reg_one_num_method() {
        let result = reg_reg_num(DrawSprite, 4, 5, 10);
        assert_eq!(result, Ok(0xD45A));
    }

    #[test]
    fn check_one_reg_one_num_method() {
        let result = reg_num(SetRegFromNum, 4, 45);
        assert_eq!(result, Ok(0x642D));
    }

    #[test]
    fn check_one_reg_method() {
        let result = reg(SkipIfKeyPressed, 3);
        assert_eq!(result, Ok(0xE39E));
    }

    #[test]
    fn check_no_param_method() {
        let result = no_param(ClearDisplay);
        assert_eq!(result, Ok(0x00E0));
    }

    #[test]
    fn check_address_method() {
        let result = address(Jump, 0x45);
        assert_eq!(result, Ok(0x1045));

        let result = address_unchecked(Jump, 0xF045);
        assert_eq!(result, Ok(0x1045));
    }
}
