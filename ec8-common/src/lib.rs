//! EmmaChip-8 Common
//!
//! Contains values and methods useful in multiple EmmaChip-8 crates
//!
//! Registers
//! I = Memory Register, 16 bit
//! Vx = Register 0-F, 8 bit (VF is the flag register and should not be written to)
//!
//! Inaccessible registers
//! PC = Program Counter, 16 bit

pub mod describe;
pub mod error;
pub mod graphics;
pub mod nibbler;
pub mod opcodes;

pub const MAX_ADDRESS: u16 = 0xFFF;
pub const PROG_START_ADDRESS: u16 = 0x200;
pub const PROG_END_ADDRESS: u16 = 0xE8F;
pub const MAX_STACK_COUNT: usize = 40;
pub const ALPHA_START_ADDRESS: u16 = 0x000;
pub const ALPHA_BYTES: usize = 5;
pub const ALPHA_COUNT: usize = 16;
pub const MEMORY_SIZE: usize = 4096;
pub const REGISTER_COUNT: usize = 16;
pub const MAX_PROG_SIZE: usize = (PROG_END_ADDRESS - PROG_START_ADDRESS) as usize;
pub const MAX_X: usize = 0x3F;
pub const MAX_Y: usize = 0x1F;
pub const PIXEL_COUNT: usize = MAX_X * MAX_Y;
pub const BUTTON_COUNT: usize = 16;
pub const REG_FLAG: usize = 15;

/// Op Codes for the EmmaChip-8
///
/// Legend
/// nnn = Address 0-FFF
/// nn = 8 bit literal number 0-FF
/// n = 4 bit literal number 0-F
/// x or y = Register number 0-F
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum OpCodes {
    /// 0nnn
    SysCall,
    /// 00E0
    ///
    /// Clears the display
    ClearDisplay,
    /// 00EE
    ///
    /// Return from call
    Return,
    /// 1nnn
    ///
    /// Jump to nnn
    Jump,
    /// 2nnn
    ///
    /// Jump to nnn as subroutine
    Call,
    /// 3xnn
    ///
    /// Skip next instruction if Vx == nn
    SkipIfEqualNum,
    /// 4xnn
    ///
    /// Skip next instruction if Vx != nn
    SkipIfNotEqualNum,
    /// 5xy0
    ///
    /// Skip next instruction if Vx == Vy
    SkipIfEqualReg,
    /// 6xnn
    ///
    /// Set Vx = nn
    SetRegFromNum,
    /// 7xnn
    ///
    /// Set Vx = Vx + nn
    AddNumToReg,
    /// 8xy0
    ///
    /// Set Vx = Vy
    SetRegFromReg,
    /// 8xy1
    ///
    /// Set Vx = Vx | Vy
    BitwiseOr,
    /// 8xy2
    ///
    /// Set Vx = Vx & Vy
    BitwiseAnd,
    /// 8xy3
    ///
    /// Set Vx = Vx ^ Vy
    BitwiseXor,
    /// 8xy4
    ///
    /// Set Vx = Vx + Vy
    AddReg,
    /// 8xy5
    /// 
    /// Set Vx = Vx - Vy
    SubRightReg,
    /// 8xy6
    ShiftRight,
    /// 8xy7
    /// 
    /// Set Vx = Vy - Vx
    SubLeftReg,
    /// 8xye
    ShiftLeft,
    /// 9xy0
    SkipIfNotEqualReg,
    /// Annn
    SetMemReg,
    /// Bnnn
    /// 
    /// Set I = nnn
    JumpOffset,
    /// Cxnn
    ///
    /// Set Vx = rand(0..=255) & nn
    SetRegRand,
    /// Dxyn
    /// 
    /// Draw sprite at Vx,Vy with n rows from I
    DrawSprite,
    /// Ex9E
    SkipIfKeyPressed,
    /// ExA1
    SkipIfKeyNotPressed,
    /// Fx07
    SetRegFromTimer,
    /// Fx0A
    /// 
    /// Blocks execution until a key is pressed
    /// Key is stored in Vx
    WaitForKey,
    /// Fx15
    SetDelayTimer,
    /// Fx18
    SetSoundTimer,
    /// Fx1E
    ///
    /// Set I = I + Vx
    AddMemReg,
    /// Fx29
    /// 
    /// Set I to address of sprite for hex digit
    SetMemRegToDigitSprite,
    /// Fx30
    /// 
    /// Set I to address of sprite for ASCII value
    SetMemRegToAsciiSprite,
    /// Fx33
    /// 
    /// Store BCD representation of Vx at I
    StoreBcd,
    /// Fx55
    /// 
    /// Store register values starting at I, up to Vx
    StoreRegs,
    /// Fx65
    /// 
    /// Load register values starting at I, up to Vx
    LoadRegs,
}

impl OpCodes {
    pub fn is_ec8_only(&self) -> bool {
        matches!(self, OpCodes::SetMemRegToAsciiSprite)
    }
}