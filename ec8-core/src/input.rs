use crate::input::Key::*;
use crate::EmmaChip8;
use crate::State::{Running, WaitingForKey};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Key {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    KA,
    KB,
    KC,
    KD,
    KE,
    KF,
}

/// Hardware layout
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F
impl Key {
    pub fn index(&self) -> usize {
        *self as usize
    }

    /// 0 -> K0,
    /// 1 -> K1,
    /// A -> KA,
    /// etc
    pub fn from_direct(chr: char) -> Option<Key> {
        match chr.to_ascii_lowercase() {
            '0' => Some(K0),
            '1' => Some(K1),
            '2' => Some(K2),
            '3' => Some(K3),
            '4' => Some(K4),
            '5' => Some(K5),
            '6' => Some(K6),
            '7' => Some(K7),
            '8' => Some(K8),
            '9' => Some(K9),
            'a' => Some(KA),
            'b' => Some(KB),
            'c' => Some(KC),
            'd' => Some(KD),
            'e' => Some(KE),
            'f' => Some(KF),
            _ => None,
        }
    }

    /// For layout
    /// 1 2 3 4
    /// Q W E R
    /// A S D F
    /// Z X C V
    pub fn from_lefthand_layout(chr: char) -> Option<Key> {
        match chr.to_ascii_lowercase() {
            '1' => Some(K1),
            '2' => Some(K2),
            '3' => Some(K3),
            '4' => Some(KC),
            'q' => Some(K4),
            'w' => Some(K5),
            'e' => Some(K6),
            'r' => Some(KD),
            'a' => Some(K7),
            's' => Some(K8),
            'd' => Some(K9),
            'f' => Some(KE),
            'z' => Some(KA),
            'x' => Some(K0),
            'c' => Some(KB),
            'v' => Some(KF),
            _ => None,
        }
    }
}

impl EmmaChip8 {
    pub fn on_key_pressed(&mut self, key: Key) {
        if let WaitingForKey(reg) = self.state {
            self.state = Running;
            self.registers[reg as usize] = key.index() as u8;
        }
        self.keys[key.index()] = true;
    }

    pub fn on_key_released(&mut self, key: Key) {
        self.keys[key.index()] = false;
    }
}
