use crate::error::ECoreError::ProgramTooLarge;
use crate::error::ECoreResult;
use crate::State::{Running, Waiting};
use ec8_common::graphics::ALPHA_MEMORY;
use ec8_common::*;
#[cfg(feature = "logging")]
use log::info;
use std::collections::VecDeque;

pub mod error;
pub mod input;
pub mod runtime;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum State {
    Waiting,
    Running,
    StackOverflow,
    InvalidOpcode,
    StackEmpty,
    WaitingForKey(u8),
}

#[derive(Debug, Clone)]
pub struct EmmaChip8 {
    pub pc: u16,
    pub memory: [u8; MEMORY_SIZE],
    pub registers: [u8; REGISTER_COUNT],
    pub stack: VecDeque<u16>,
    pub mem_reg: u16,
    pub delay: u8,
    pub sound: u8,
    pub output: [bool; PIXEL_COUNT],
    pub state: State,
    pub keys: [bool; BUTTON_COUNT],
    pub dirty: bool,
}

impl EmmaChip8 {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: VecDeque::new(),
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            mem_reg: 0,
            delay: 0,
            sound: 0,
            output: [false; PIXEL_COUNT],
            state: Waiting,
            keys: [false; BUTTON_COUNT],
            dirty: false,
        }
    }
}

impl EmmaChip8 {
    /// Load program into memory and reset all registers
    pub fn load_program(&mut self, data: &[u8]) -> ECoreResult<()> {
        #[cfg(feature = "logging")]
        info!("Program loading...");

        if data.len() > MAX_PROG_SIZE {
            return Err(ProgramTooLarge);
        }

        let mut memory = [0; MEMORY_SIZE];
        for (i, byte) in ALPHA_MEMORY.iter().enumerate() {
            memory[i] = *byte;
        }
        for (i, byte) in data.iter().enumerate() {
            memory[i + (PROG_START_ADDRESS as usize)] = *byte;
        }

        self.memory = memory;
        self.pc = PROG_START_ADDRESS;
        self.mem_reg = PROG_START_ADDRESS;
        self.sound = 0;
        self.delay = 0;
        self.registers = [0; REGISTER_COUNT];
        self.output = [false; PIXEL_COUNT];
        self.state = Running;
        self.keys = [false; BUTTON_COUNT];
        self.dirty = true;

        #[cfg(feature = "logging")]
        info!("Program loaded");

        Ok(())
    }
}
