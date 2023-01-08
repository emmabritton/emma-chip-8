use crate::{ALPHA_BYTES, ALPHA_START_ADDRESS};

#[rustfmt::skip]
pub const ALPHA_MEMORY: [u8; 315] = [
    // 4x5
    0xF0, 0x90, 0x90, 0x90, 0xF0, //0
    0x20, 0x60, 0x20, 0x20, 0x70, //1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
    0x90, 0x90, 0xF0, 0x10, 0x10, //4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
    0xF0, 0x10, 0x20, 0x40, 0x40, //7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
    0xF0, 0x90, 0xF0, 0x90, 0x90, //A,
    0xE0, 0x90, 0xE0, 0x90, 0xE0, //B,
    0xF0, 0x80, 0x80, 0x80, 0xF0, //C,
    0xE0, 0x90, 0x90, 0x90, 0xE0, //D,
    0xF0, 0x80, 0xF0, 0x80, 0xF0, //E,
    0xF0, 0x80, 0xF0, 0x80, 0x80, //F,
    0xF0, 0x40, 0xB0, 0x90, 0xF0, //G,
    0x90, 0x90, 0xF0, 0x90, 0x90, //H,
    0xF0, 0x20, 0x20, 0x20, 0xF0, //I,
    0xF0, 0x20, 0x20, 0xA0, 0xE0, //J,
    0x90, 0x90, 0xE0, 0x90, 0x90, //K,
    0x40, 0x40, 0x40, 0x40, 0xF0, //L,
    0x90, 0xF0, 0x90, 0x90, 0x90, //M,
    0x60, 0xD0, 0xB0, 0x90, 0x90, //N,
    0x60, 0x90, 0x90, 0x90, 0x60, //O,
    0xE0, 0x90, 0xE0, 0x40, 0x40, //P,
    0x60, 0x90, 0x90, 0xB0, 0x70, //Q,
    0xE0, 0x90, 0xE0, 0xA0, 0x90, //R,
    0x70, 0x80, 0x60, 0x10, 0xE0, //S,
    0x70, 0x20, 0x20, 0x20, 0x20, //T,
    0x90, 0x90, 0x90, 0x90, 0xf0, //U,
    0x50, 0x50, 0x50, 0x50, 0x20, //V,
    0x90, 0x90, 0x90, 0xF0, 0x90, //W,
    0x90, 0x90, 0x60, 0x90, 0x90, //X,
    0x50, 0x50, 0x50, 0x20, 0x20, //Y,
    0xF0, 0x10, 0x60, 0x80, 0xF0, //Z,
    0x20, 0x20, 0x20, 0x00, 0x20, //\!,
    0x60, 0x10, 0x20, 0x00, 0x20, //?,
    0x90, 0x60, 0xF0, 0x60, 0x90, //*,
    0x00, 0x20, 0x70, 0x20, 0x00, //+,
    0x00, 0x00, 0x70, 0x00, 0x00, //-,
    0x00, 0x70, 0x00, 0x70, 0x00, //=,
    0x20, 0x40, 0x80, 0x40, 0x20, //<,
    0x40, 0x20, 0x10, 0x20, 0x40, //>,
    0x00, 0x20, 0x00, 0x20, 0x00, //:,
    0x20, 0x20, 0x00, 0x00, 0x00, //',
    0x50, 0x50, 0x00, 0x00, 0x00, //",
    0x20, 0x40, 0x40, 0x40, 0x20, //(,
    0x40, 0x20, 0x20, 0x20, 0x40, //),
    0x70, 0xA0, 0x60, 0x50, 0xE0, //$,
    0x40, 0xA0, 0x40, 0xA0, 0x50, //&,
    0x60, 0x90, 0xB0, 0x80, 0x70, //@,
    0x10, 0x20, 0x40, 0x80, 0x00, //\/,
    0x80, 0x40, 0x20, 0x10, 0x00, //\,
    0x00, 0x00, 0x00, 0x00, 0xF0, //_,
    0x70, 0x40, 0x40, 0x40, 0x70, //[,
    0xE0, 0x20, 0x20, 0x20, 0xE0, //],
    0x20, 0x40, 0xC0, 0x40, 0x20, //{,
    0x40, 0x20, 0x30, 0x20, 0x40, //},
    0x20, 0x50, 0x00, 0x00, 0x00, //^,
    0x90, 0x20, 0x40, 0x90, 0x00, //%,
    0x00, 0x20, 0x00, 0x20, 0x20, //;,
    0x50, 0xF0, 0x50, 0xF0, 0x50, //#,


    // 8x8
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //0
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //1
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //2
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //3
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //4
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //5
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //6
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //7
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //8
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //9
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //A
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //B
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //C
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //D
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //E
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //F
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //G
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //H
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //I
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //J
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //K
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //L
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //M
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //N
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //O
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //P
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //Q
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //R
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //S
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //T
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //U
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //V
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //W
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //X
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //Y
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //Z
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //\!
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //?
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //*
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //+
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //-
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //=
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //<
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //>
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //:
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //'
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //"
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //(
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //)
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //$
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //&
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //@
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //\/
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //\
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //_
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //[
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //]
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //{
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //}
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //^
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //%
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //#
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //~
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //`
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //£
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //¥
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //°
    // 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //|
];

pub fn alpha_index(chr: char) -> Option<usize> {
    let chr = chr.to_ascii_lowercase();
    if chr.is_ascii_digit() {
        chr.to_digit(10).map(|num| num as usize)
    } else if chr.is_alphabetic() {
        Some(((chr as u8) - 97 + 10) as usize)
    } else {
        let symbols = [
            '!', '?', '*', '+', '-', '=', '<', '>', ':', '\'', '"', '(', ')', '$', '&', '@', '/',
            '\\', '_', '[', ']', '{', '}', '^', '%', ';', '#',
        ];
        symbols.iter().position(|c| c == &chr).map(|i| i + 36)
    }
}

pub fn alpha_addr(chr: char) -> Option<u16> {
    alpha_index(chr).map(|i| ((i * ALPHA_BYTES) as u16) + ALPHA_START_ADDRESS)
}

#[cfg(test)]
mod test {
    use crate::graphics::alpha_addr;

    #[test]
    fn check_alpha_addr() {
        assert_eq!(alpha_addr('0'), Some(0));
        assert_eq!(alpha_addr('a'), Some(50));
        assert_eq!(alpha_addr('A'), Some(50));
        assert_eq!(alpha_addr('!'), Some(180));
    }
}