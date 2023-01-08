use crate::State::{Running, WaitingForKey};
use crate::{EmmaChip8, State};
use ec8_common::nibbler::{Masher, Nibbler};
use ec8_common::{
    opcodes, OpCodes, ALPHA_BYTES, ALPHA_START_ADDRESS, MAX_STACK_COUNT, MAX_X, PIXEL_COUNT,
    REG_FLAG,
};
#[cfg(feature = "logging")]
use log::{debug, error, info, warn};

impl EmmaChip8 {
    pub fn run(&mut self) {
        if self.state == Running {
            let bytes = self.read_two_bytes(self.pc);
            #[cfg(feature = "logging")]
            debug!("Preparing to execute {:02X} {:02X}", bytes[0], bytes[1]);
            match opcodes::from_bytes(bytes) {
                None => {
                    self.state = State::InvalidOpcode;
                    #[cfg(feature = "logging")]
                    error!("State set to InvalidOpcode");
                }
                Some(opcode) => self.execute(opcode, bytes),
            }
        } else {
            #[cfg(feature = "logging")]
            warn!("Not running, state is {:?}", self.state);
        }
    }

    /// Execute Opcode with data
    /// Returns true if PC was updated by the operation
    fn execute(&mut self, opcode: OpCodes, bytes: [u8; 2]) {
        #[cfg(feature = "logging")]
        debug!("Decoded as {:?}", opcode);
        self.pc += 2;
        #[cfg(feature = "logging")]
        let mut debug_data = 0;
        //amount to change self.pc
        // 2 by default as pc was +=2 above
        // 0 for jumps
        // 4 for skips
        #[cfg(feature = "logging")]
        let mut debug_pc = 2;
        #[cfg(feature = "logging")]
        let pre_op = self.clone();
        let x = bytes[0].second_nibble();
        let y = bytes[1].first_nibble_shifted();
        match opcode {
            OpCodes::SysCall => {/*do nothing, not supported*/},
            OpCodes::ClearDisplay => self.output.fill(false),
            OpCodes::Return => match self.stack.pop_back() {
                None => self.state = State::StackEmpty,
                Some(addr) => {
                    #[cfg(feature = "logging")]
                    {
                        debug_pc = 0;
                        debug_data = self.pc - 2;
                    }
                    self.pc = addr
                }
            },
            OpCodes::Jump => {
                #[cfg(feature = "logging")]
                {
                    debug_pc = 0;
                    debug_data = self.pc - 2;
                }
                self.pc = bytes.mash_to_12bits();
            }
            OpCodes::Call => {
                if self.stack.len() < MAX_STACK_COUNT {
                    self.stack.push_back(self.pc);
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.pc - 2;
                        debug_pc = 0;
                    }
                    self.pc = bytes.mash_to_12bits()
                } else {
                    self.state = State::StackOverflow;
                }
            }
            OpCodes::SkipIfEqualNum => {
                if self.read_reg(x) == bytes[1] {
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.read_next_instr_u16();
                        debug_pc = 4;
                    }
                    self.pc += 2;
                }
            }
            OpCodes::SkipIfNotEqualNum => {
                if self.read_reg(x) != bytes[1] {
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.read_next_instr_u16();
                        debug_pc = 4;
                    }
                    self.pc += 2;
                }
            }
            OpCodes::SkipIfEqualReg => {
                if self.read_reg(x) == self.read_reg(y) {
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.read_next_instr_u16();
                        debug_pc = 4;
                    }
                    self.pc += 2;
                }
            }
            OpCodes::SetRegFromNum => self.set_reg(x, bytes[1]),
            OpCodes::AddNumToReg => {
                let target = x;
                let value = self.read_reg(target);
                self.set_reg(target, value.overflowing_add(bytes[1]).0);
            }
            OpCodes::SetRegFromReg => self.set_reg(x, self.read_reg(y)),
            OpCodes::BitwiseOr => {
                self.set_reg(x, self.read_reg(x) | self.read_reg(y));
            }
            OpCodes::BitwiseAnd => {
                self.set_reg(x, self.read_reg(x) & self.read_reg(y));
            }
            OpCodes::BitwiseXor => {
                self.set_reg(x, self.read_reg(x) ^ self.read_reg(y));
            }
            OpCodes::AddReg => {
                let (result, overflowed) = self.read_reg(x).overflowing_add(self.read_reg(y));
                self.set_reg(x, result);
                self.set_flag(overflowed);
            }
            OpCodes::SubRightReg => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.set_flag(vx > vy);
                self.set_reg(x, vx.wrapping_sub(vy));
            }
            OpCodes::ShiftRight => {
                let value = self.read_reg(x);
                self.registers[REG_FLAG] = value & 0x01;
                self.set_reg(x, value >> 1);
            }
            OpCodes::SubLeftReg => {
                let vx = self.read_reg(x);
                let vy = self.read_reg(y);
                self.set_flag(vy > vx);
                self.set_reg(x, vy.wrapping_sub(vx));
            }
            OpCodes::ShiftLeft => {
                let value = self.read_reg(x);
                self.registers[REG_FLAG] = value >> 7;
                self.set_reg(x, value << 1);
            }
            OpCodes::SkipIfNotEqualReg => {
                if self.read_reg(x) != self.read_reg(y) {
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.read_next_instr_u16();
                        debug_pc = 4;
                    }
                    self.pc += 2;
                }
            }
            OpCodes::SetMemReg => self.mem_reg = bytes.mash_to_12bits(),
            OpCodes::JumpOffset => {
                #[cfg(feature = "logging")]
                {
                    debug_data = self.pc - 2;
                    debug_pc = 0;
                }
                self.pc = (self.read_reg(0) as u16) + bytes.mash_to_12bits();
            }
            OpCodes::SetRegRand => {
                let rand = fastrand::u8(..);
                #[cfg(feature = "logging")]
                {
                    debug_data = rand as u16;
                }
                self.set_reg(x, rand & bytes[1]);
            }
            OpCodes::DrawSprite => {
                self.draw_sprite(self.read_reg(x), self.read_reg(y), bytes[1].second_nibble())
            }
            OpCodes::SkipIfKeyPressed => {
                let key = self.read_reg(x);
                if self.keys[key as usize] {
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.read_next_instr_u16();
                        debug_pc = 4;
                    }
                    self.pc += 2;
                }
            }
            OpCodes::SkipIfKeyNotPressed => {
                let key = self.read_reg(x);
                if !self.keys[key as usize] {
                    #[cfg(feature = "logging")]
                    {
                        debug_data = self.read_next_instr_u16();
                        debug_pc = 4;
                    }
                    self.pc += 2;
                }
            }
            OpCodes::SetRegFromTimer => {
                self.set_reg(x, self.delay);
                #[cfg(feature = "logging")]
                {
                    debug_data = self.delay as u16;
                }
            }
            OpCodes::WaitForKey => self.state = WaitingForKey(x),
            OpCodes::SetDelayTimer => self.delay = self.read_reg(x),
            OpCodes::SetSoundTimer => self.sound = self.read_reg(x),
            OpCodes::AddMemReg => {
                let num = self.read_reg(x);
                self.mem_reg += num as u16;
            }
            OpCodes::SetMemRegToDigitSprite => {
                let digit = self.read_reg(x).second_nibble();
                self.mem_reg = ALPHA_START_ADDRESS + ALPHA_BYTES as u16 * digit as u16;
            }
            OpCodes::StoreBcd => {
                let value = self.read_reg(x);
                self.memory[self.mem_reg as usize] = value / 100;
                self.memory[self.mem_reg as usize + 1] = (value / 10) % 10;
                self.memory[self.mem_reg as usize + 2] = value % 10;
            }
            OpCodes::StoreRegs => {
                let stop_at = x as usize;
                for i in 0..=stop_at {
                    let addr = (self.mem_reg as usize) + i;
                    self.memory[addr] = self.registers[i];
                }
            }
            OpCodes::LoadRegs => {
                let stop_at = x as usize;
                for i in 0..=stop_at {
                    let addr = (self.mem_reg as usize) + i;
                    self.registers[i] = self.memory[addr];
                }
            }
        }
        #[cfg(feature = "logging")]
        info!(
            "{}",
            opcode.describe(
                bytes,
                pre_op.registers,
                pre_op.mem_reg,
                self.registers,
                self.mem_reg,
                self.pc - debug_pc,
                debug_data,
                debug_pc
            )
        );
    }

    #[inline(always)]
    fn set_flag(&mut self, set: bool) {
        if set {
            self.registers[REG_FLAG] = 1;
        } else {
            self.registers[REG_FLAG] = 0;
        }
    }

    #[inline(always)]
    fn read_reg(&self, reg: u8) -> u8 {
        self.registers[reg as usize]
    }

    #[inline(always)]
    fn set_reg(&mut self, reg: u8, value: u8) {
        self.registers[reg as usize] = value;
    }

    #[inline(always)]
    fn read_two_bytes(&self, addr: u16) -> [u8; 2] {
        let addr = addr as usize;
        [self.memory[addr], self.memory[addr + 1]]
    }

    #[cfg(feature = "logging")]
    fn read_next_instr_u16(&self) -> u16 {
        u16::from_be_bytes(self.read_two_bytes(self.pc))
    }

    fn draw_sprite(&mut self, x: u8, y: u8, rows: u8) {
        #[cfg(feature = "logging")]
        debug!(
            "Drawing sprite at {x}, {y}, with {rows} rows from {:03X}",
            self.mem_reg
        );
        self.dirty = true;
        let mut collision = false;
        for row in 0..(rows as usize) {
            let addr = self.mem_reg as usize + row;
            let pixels = self.memory[addr];
            let py = y as usize + row;
            for i in 0..8 {
                let px = x as usize + i;
                let set_pixel = (pixels >> (7 - i) & 0x01) == 1;
                let output_idx = (py * MAX_X + px).min(PIXEL_COUNT - 1);
                let old_value = self.output[output_idx];
                self.output[output_idx] ^= set_pixel;
                if old_value != self.output[output_idx] {
                    collision = true;
                }
            }
        }
        self.set_flag(collision);
    }
}

#[cfg(test)]
mod test {
    use crate::EmmaChip8;
    use crate::State::Running;
    use ec8_common::{ALPHA_BYTES, MAX_X, PIXEL_COUNT};

    #[test]
    fn check_basics() {
        let mut ec8 = EmmaChip8::new();
        //Jump to 0x204
        //Set R2 = 1
        //Set R0 = R2
        //Set R0 = R0 + R0
        ec8.load_program(&[0x12, 0x04, 0x0, 0x0, 0x62, 0x01, 0x80, 0x20, 0x80, 0x04])
            .unwrap();
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x204);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.registers[2], 1);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.registers[0], 1);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.registers[0], 2);
    }

    #[test]
    fn check_set_sprite() {
        let mut ec8 = EmmaChip8::new();
        ec8.load_program(&[0xF0, 0x29, 0x62, 0x08, 0xF2, 0x29])
            .unwrap();
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.mem_reg, 0);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.mem_reg, 0);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.mem_reg, 8 * ALPHA_BYTES as u16);
    }

    #[test]
    fn check_output() {
        let mut ec8 = EmmaChip8::new();
        ec8.load_program(&[0xA0, 0x0, 0xD0, 0x05]).unwrap();
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.mem_reg, 0);
        ec8.run();
        assert_eq!(ec8.state, Running);
        let mut expected = [false; PIXEL_COUNT];
        expected[0] = true;
        expected[1] = true;
        expected[2] = true;
        expected[3] = true;
        expected[MAX_X] = true;
        expected[MAX_X + 3] = true;
        expected[MAX_X * 2] = true;
        expected[MAX_X * 2 + 3] = true;
        expected[MAX_X * 3] = true;
        expected[MAX_X * 3 + 3] = true;
        expected[MAX_X * 4] = true;
        expected[MAX_X * 4 + 1] = true;
        expected[MAX_X * 4 + 2] = true;
        expected[MAX_X * 4 + 3] = true;
        assert_eq!(ec8.output, expected);
    }

    #[test]
    fn check_sub() {
        let mut ec8 = EmmaChip8::new();
        ec8.load_program(&[
            0x6A, 0xFF, 0x6B, 0xF1, 0x62, 20, 0x63, 30, 0x8A, 0xB5, 0x82, 0x37, 0x3A, 0x0E, 0xAF,
            0xFF,
        ])
        .unwrap();
        ec8.run();
        ec8.run();
        ec8.run();
        ec8.run();
        ec8.run();
        ec8.run();
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.registers[0x0A], 0x0E);
        assert_eq!(ec8.registers[2], 10);
        assert_eq!(ec8.pc, 0x200 + 16);
    }

    #[test]
    fn check_skip() {
        let mut ec8 = EmmaChip8::new();
        ec8.load_program(&[
            0x60, 0xFF, 0xF0, 0x15, 0x60, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x60, 0x00, 0x30, 0x01,
            0x30, 0x00, 0x13, 0x92,
        ])
        .unwrap();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x200);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x202);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x204);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x206);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x208);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x20A);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x20C);
        ec8.run();
        assert_eq!(ec8.state, Running);
        assert_eq!(ec8.pc, 0x20E);
    }
}
