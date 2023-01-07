use crate::OpCodes;
use std::fmt::{Display, Formatter};

pub type ECommonResult<T> = Result<T, ECommonError>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ECommonError {
    AddressOutOfRange(u16),
    InvalidOpCode(OpCodes),
    InvalidRegister(u8),
    NumberTooLarge(u8),
}

impl Display for ECommonError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ECommonError::AddressOutOfRange(addr) => {
                write!(f, "address is too large: {:#06X}, max: 0xFFF", addr)
            }
            ECommonError::InvalidOpCode(opcode) => write!(f, "invalid params for {:?}", opcode),
            ECommonError::InvalidRegister(reg) => write!(f, "invalid register {:#04X}", reg),
            ECommonError::NumberTooLarge(num) => {
                write!(f, "number is too large {num}|{:#04X}, max is 16|0x0F", num)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::ECommonError::*;
    use crate::OpCodes::ClearDisplay;

    #[test]
    fn check_error_strings() {
        assert_eq!(
            AddressOutOfRange(0x451B).to_string(),
            "address is too large: 0x451B, max: 0xFFF"
        );
        assert_eq!(
            InvalidOpCode(ClearDisplay).to_string(),
            "invalid params for ClearDisplay"
        );
        assert_eq!(InvalidRegister(17).to_string(), "invalid register 0x11");
        assert_eq!(
            NumberTooLarge(64).to_string(),
            "number is too large 64|0x40, max is 16|0x0F"
        );
    }
}
