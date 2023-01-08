use crate::nibbler::{Masher, Nibbler};
use crate::opcodes::from_bytes;
use crate::{OpCodes, REGISTER_COUNT};

impl OpCodes {
    pub fn simple_describe(&self, bytes: [u8; 2]) -> String {
        let x = format!("V{:01X}", bytes[0].second_nibble());
        let y = format!("V{:01X}", bytes[1].first_nibble_shifted());
        let n = format!("{:01X}", bytes[1].second_nibble());
        let nn = format!("{:02X}", bytes[1]);
        let nnn = format!("{:03X}", u16::from_be_bytes(bytes) & 0xFFF);
        match self {
            OpCodes::SysCall => format!("SysCall to {nnn} (Unsupported)"),
            OpCodes::ClearDisplay => "Clear the display".to_string(),
            OpCodes::Return => "Return from subroutine".to_string(),
            OpCodes::Jump => format!("Jump to {nnn}"),
            OpCodes::Call => format!("Call subroutine at {nnn}"),
            OpCodes::SkipIfEqualNum => format!("Skip if {x} == {nn}"),
            OpCodes::SkipIfNotEqualNum => format!("Skip if {x} != {nn}"),
            OpCodes::SkipIfEqualReg => format!("Skip if {x} == {y}"),
            OpCodes::SetRegFromNum => format!("Set {x} to {nn}"),
            OpCodes::AddNumToReg => format!("Set {x} to {x} + {nn}"),
            OpCodes::SetRegFromReg => format!("Set {x} to {y}"),
            OpCodes::BitwiseOr => format!("Set {x} to {x} | {y}"),
            OpCodes::BitwiseAnd => format!("Set {x} to {x} & {y}"),
            OpCodes::BitwiseXor => format!("Set {x} to {x} ^ {y}"),
            OpCodes::AddReg => format!("Set {x} to {x} + {y}"),
            OpCodes::SubRightReg => format!("Set {x} to {x} - {y}"),
            OpCodes::ShiftRight => format!("Set VF to first bit of {x}, set {x} to {x} >> 1"),
            OpCodes::SubLeftReg => format!("Set {x} to {y} - {x}"),
            OpCodes::ShiftLeft => format!("Set VF to last bit of {x}, set {x} to {x} << 1"),
            OpCodes::SkipIfNotEqualReg => format!("Skip if {x} != {y}"),
            OpCodes::SetMemReg => format!("Set I to {nnn}"),
            OpCodes::JumpOffset => format!("Jump to {nnn} + V0"),
            OpCodes::SetRegRand => format!("Set {x} to rand(0..=255) & {nn}"),
            OpCodes::DrawSprite => format!("Draw sprite at {x},{y} with {n} rows from I"),
            OpCodes::SkipIfKeyPressed => format!("Skipping if key in {x} is pressed"),
            OpCodes::SkipIfKeyNotPressed => format!("Skipping if key in {x} is not pressed"),
            OpCodes::SetRegFromTimer => format!("Set {x} to delay timer"),
            OpCodes::WaitForKey => format!("Wait for key press, and store it in {x}"),
            OpCodes::SetDelayTimer => format!("Set delay timer to {x}"),
            OpCodes::SetSoundTimer => format!("Set sound timer to {x}"),
            OpCodes::AddMemReg => format!("Set I to I + {x}"),
            OpCodes::SetMemRegToDigitSprite => format!("Set I to addr of digit in {x}"),
            OpCodes::SetMemRegToAsciiSprite => format!("Set I to addr of ASCII in {x}"),
            OpCodes::StoreBcd => format!("Store {x} as BCD starting at I"),
            OpCodes::StoreRegs => format!("Store regs from V0 to {x} in memory starting at I"),
            OpCodes::LoadRegs => format!("Load regs from V0 to {x} from memory starting at I"),
        }
    }
    
    #[allow(clippy::too_many_arguments)] //it's a complicated debug method
    pub fn describe(
        &self,
        bytes: [u8; 2],
        pre_registers: [u8; REGISTER_COUNT],
        pre_mem_reg: u16,
        post_registers: [u8; REGISTER_COUNT],
        post_mem_reg: u16,
        mut pc: u16,
        data: u16,
        pc_delta: u16,
    ) -> String {
        let pc_set = pc_delta != 2;
        let data_bytes = data.to_be_bytes();
        let next_instr = if pc_set {
            from_bytes(data_bytes)
                .map(|op| {
                    op.describe(
                        data_bytes,
                        pre_registers,
                        pre_mem_reg,
                        post_registers,
                        post_mem_reg,
                        pc + 2,
                        0,
                        2,
                    )
                })
                .unwrap_or(format!("DATA {:04X}", data))
        } else {
            "not skipped".to_string()
        };
        let x = format!("V{:01X}", bytes[0].second_nibble());
        let y = format!("V{:01X}", bytes[1].first_nibble_shifted());
        let n = format!("{:01X}", bytes[1].second_nibble());
        let pre_vx = format!(
            "{x} ({:02X})",
            pre_registers[(bytes[0].second_nibble() as usize).min(15)]
        );
        let post_vx = format!(
            "{x} ({:02X})",
            post_registers[(bytes[0].second_nibble() as usize).min(15)]
        );
        let pre_vy = format!(
            "{y} ({:02X})",
            pre_registers[(bytes[1].first_nibble_shifted() as usize).min(15)]
        );
        let nn = format!("{:02X}", bytes[1]);
        let addr = format!("{:03X}", bytes.mash_to_12bits());
        if pc_delta == 0 {
            pc = data
        }
        let pc = format!("{:04X}", pc);
        let pre_mem_reg = format!("I ({:02X})", pre_mem_reg);
        let post_mem_reg = format!("I ({:02X})", post_mem_reg);
        let data_byte = format!("{:02X}", data);
        let data_addr = format!("{:03X}", data);
        let regs = || {
            pre_registers
                .iter()
                .map(|value| format!("{:02X}", value))
                .collect::<Vec<String>>()
                .join(", ")
        };
        let next_instr = format!("\n  {next_instr}");
        let text = match self {
            OpCodes::SysCall => format!("SysCall to {addr} (Unsupported)"),
            OpCodes::ClearDisplay => "Clear the display".to_string(),
            OpCodes::Return => format!("Return from {data_addr}"),
            OpCodes::Jump => format!("Jump to {addr}"),
            OpCodes::Call => format!("Call subroutine at {addr}"),
            OpCodes::SkipIfEqualNum => format!("Skipping if {pre_vx} == {nn}{next_instr}"),
            OpCodes::SkipIfNotEqualNum => format!("Skipping if {pre_vx} != {nn}{next_instr}"),
            OpCodes::SkipIfEqualReg => format!("Skipping if {pre_vx} == {pre_vy}{next_instr}"),
            OpCodes::SetRegFromNum => format!("Set {x} to {nn}"),
            OpCodes::AddNumToReg => format!("Set {post_vx} to {pre_vx} + {nn}"),
            OpCodes::SetRegFromReg => format!("Set {x} from {pre_vy}"),
            OpCodes::BitwiseOr => format!("Set {post_vx} to {pre_vx} | {pre_vy}"),
            OpCodes::BitwiseAnd => format!("Set {post_vx} to {pre_vx} & {pre_vy}"),
            OpCodes::BitwiseXor => format!("Set {post_vx} to {pre_vx} ^ {pre_vy}"),
            OpCodes::AddReg => format!("Set {post_vx} to {pre_vx} + {pre_vy}"),
            OpCodes::SubRightReg => format!("Set {post_vx} to {pre_vx} - {pre_vy}"),
            OpCodes::ShiftRight => {
                format!(
                    "Set {post_vx} to {pre_vx} >> 1, set VF ({:02X}) to first bit of {pre_vx}",
                    post_registers[15]
                )
            }
            OpCodes::SubLeftReg => format!("Set {post_vx} to {pre_vy} - {pre_vx}"),
            OpCodes::ShiftLeft => {
                format!(
                    "Set {post_vx} to {pre_vx} << 1, set VF ({:02X}) to first bit of {pre_vx}",
                    post_registers[15]
                )
            }
            OpCodes::SkipIfNotEqualReg => format!("Skipping if {pre_vx} != {pre_vy}{next_instr}"),
            OpCodes::SetMemReg => format!("Set I to {addr}"),
            OpCodes::JumpOffset => format!("Jump to V0 ({:02X}) + {addr}", pre_registers[0]),
            OpCodes::SetRegRand => format!("Set {post_vx} to {pre_vx} + rand ({data_byte}) & {nn}"),
            OpCodes::DrawSprite => {
                format!("Draw sprite at {pre_vx},{pre_vx} with {n} rows from {pre_mem_reg}")
            }
            OpCodes::SkipIfKeyPressed => {
                format!("Skipping if key in {pre_vx} is pressed{next_instr}")
            }
            OpCodes::SkipIfKeyNotPressed => {
                format!("Skipping if key in {pre_vx} is not pressed{next_instr}")
            }
            OpCodes::SetRegFromTimer => format!("Set {x} to delay timer ({data_byte})"),
            OpCodes::WaitForKey => format!("Wait for key press, and store it in {x}"),
            OpCodes::SetDelayTimer => format!("Set delay timer to {pre_vx}"),
            OpCodes::SetSoundTimer => format!("Set sound timer to {pre_vx}"),
            OpCodes::AddMemReg => format!("Set {post_mem_reg} to {pre_mem_reg} + {pre_vx}"),
            OpCodes::SetMemRegToDigitSprite => {
                format!("Set {post_mem_reg} to addr of digit {pre_vx}")
            }
            OpCodes::SetMemRegToAsciiSprite => {
                format!("Set {post_mem_reg} to addr of ASCII {pre_vx}")
            }
            OpCodes::StoreBcd => format!("Store {pre_vx} as BCD starting at {pre_mem_reg}"),
            OpCodes::StoreRegs => format!("Store registers ({}) to {pre_mem_reg}", regs()),
            OpCodes::LoadRegs => format!("Load registers ({}) from {pre_mem_reg}", regs()),
        };
        format!("[{pc}] {:02X}{:02X} {text}", bytes[0], bytes[1])
    }
}

#[cfg(test)]
mod test {
    use crate::{OpCodes, REGISTER_COUNT};

    #[test]
    fn check_simple() {
        let output = OpCodes::Jump.simple_describe([0x13, 0x34]);
        assert_eq!(output, "Jump to 334".to_string());

        let output = OpCodes::AddNumToReg.simple_describe([0x7B, 0x9A]);
        assert_eq!(output, "Set VB to VB + 9A".to_string());
    }

    #[test]
    fn check_full() {
        let output = OpCodes::ClearDisplay.describe(
            [0x00, 0xE0],
            [0; REGISTER_COUNT],
            0x12,
            [0; REGISTER_COUNT],
            0x12,
            0x12,
            0,
            2,
        );
        assert_eq!(output, "[0012] 00E0 Clear the display");

        let output = OpCodes::SetMemReg.describe(
            [0xA0, 0x67],
            [0; REGISTER_COUNT],
            0x3AA,
            [0; REGISTER_COUNT],
            0x3AA,
            0x3AA,
            0,
            2,
        );
        assert_eq!(output, "[03AA] A067 Set I to 067");

        let output = OpCodes::AddReg.describe(
            [0x83, 0x14],
            [0, 0x67, 0, 0x34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
            [0, 0x67, 0, 0x9B, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            0,
            0x9,
            0,
            2,
        );
        assert_eq!(output, "[0009] 8314 Set V3 (9B) to V3 (34) + V1 (67)");
    }
}
