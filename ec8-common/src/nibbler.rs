pub trait Nibbler {
    fn first_nibble(&self) -> u8;

    fn first_nibble_shifted(&self) -> u8;

    fn second_nibble(&self) -> u8;
}

impl Nibbler for u8 {
    fn first_nibble(&self) -> u8 {
        0xF0 & *self
    }

    fn first_nibble_shifted(&self) -> u8 {
        (0xF0 & *self) >> 4
    }

    fn second_nibble(&self) -> u8 {
        0x0F & *self
    }
}

pub trait Masher {
    fn mash_to_12bits(&self) -> u16;
}

impl Masher for [u8; 2] {
    fn mash_to_12bits(&self) -> u16 {
        let upper = ((self[0] as u16) & 0x0F) << 8;
        upper + (self[1] as u16)
    }
}

#[cfg(test)]
mod test {
    use crate::nibbler::{Masher, Nibbler};

    #[test]
    fn check_masher() {
        let bytes = [0x45_u8, 0x32];
        let mashed = bytes.mash_to_12bits();
        assert_eq!(mashed, 0x0532);
    }

    #[test]
    fn check_first_nibble() {
        let num = 0x1F_u8;
        assert_eq!(num.first_nibble(), 0x10);
        assert_eq!(num.first_nibble_shifted(), 0x01);
    }

    #[test]
    fn check_second_nibble() {
        let num = 0x1F_u8;
        assert_eq!(num.second_nibble(), 0x0F);
    }
}
