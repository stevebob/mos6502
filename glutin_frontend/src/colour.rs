const NUM_COLOURS: usize = 64;
const COLOUR_MASK: u8 = (NUM_COLOURS as u8) - 1;
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        [$r as f32 / 255., $g as f32 / 255., $b as f32 / 255.]
    };
}
const COLOUR_TABLE: [[f32; 3]; NUM_COLOURS] = [
    /* 0x00 */ rgb!(0x65, 0x65, 0x65),
    /* 0x01 */ rgb!(0x00, 0x2D, 0x69),
    /* 0x02 */ rgb!(0x13, 0x1F, 0x7F),
    /* 0x03 */ rgb!(0x3C, 0x13, 0x7C),
    /* 0x04 */ rgb!(0x60, 0x0B, 0x62),
    /* 0x05 */ rgb!(0x73, 0x0a, 0x37),
    /* 0x06 */ rgb!(0x71, 0x0F, 0x07),
    /* 0x07 */ rgb!(0x5A, 0x1A, 0x00),
    /* 0x08 */ rgb!(0x34, 0x28, 0x00),
    /* 0x09 */ rgb!(0x0B, 0x34, 0x00),
    /* 0x0A */ rgb!(0x00, 0x3C, 0x00),
    /* 0x0B */ rgb!(0x00, 0x3D, 0x10),
    /* 0x0C */ rgb!(0x00, 0x38, 0x40),
    /* 0x0D */ rgb!(0x00, 0x00, 0x00),
    /* 0x0E */ rgb!(0x00, 0x00, 0x00),
    /* 0x0F */ rgb!(0x00, 0x00, 0x00),
    /* 0x10 */ rgb!(0xAE, 0xAE, 0xAE),
    /* 0x11 */ rgb!(0x0F, 0x63, 0xB3),
    /* 0x12 */ rgb!(0x40, 0x51, 0xD0),
    /* 0x13 */ rgb!(0x78, 0x41, 0xCC),
    /* 0x14 */ rgb!(0xA7, 0x36, 0xA9),
    /* 0x15 */ rgb!(0xC0, 0x34, 0x70),
    /* 0x16 */ rgb!(0xBD, 0x3C, 0x30),
    /* 0x17 */ rgb!(0x9F, 0x4A, 0x00),
    /* 0x18 */ rgb!(0x6D, 0x5C, 0x00),
    /* 0x19 */ rgb!(0x36, 0x6D, 0x00),
    /* 0x1A */ rgb!(0x07, 0x77, 0x04),
    /* 0x1B */ rgb!(0x00, 0x79, 0x3D),
    /* 0x1C */ rgb!(0x00, 0x72, 0x7D),
    /* 0x1D */ rgb!(0x00, 0x00, 0x00),
    /* 0x1E */ rgb!(0x00, 0x00, 0x00),
    /* 0x1F */ rgb!(0x00, 0x00, 0x00),
    /* 0x20 */ rgb!(0xFE, 0xFE, 0xFF),
    /* 0x21 */ rgb!(0x5D, 0xB3, 0xFF),
    /* 0x22 */ rgb!(0x8F, 0xA1, 0xFF),
    /* 0x23 */ rgb!(0xC8, 0x90, 0xFF),
    /* 0x24 */ rgb!(0xF7, 0x85, 0xFA),
    /* 0x25 */ rgb!(0xFF, 0x83, 0xC0),
    /* 0x26 */ rgb!(0xFF, 0x8B, 0x7F),
    /* 0x27 */ rgb!(0xEF, 0x9A, 0x49),
    /* 0x28 */ rgb!(0xBD, 0xAC, 0x2C),
    /* 0x29 */ rgb!(0x85, 0xBC, 0x2F),
    /* 0x2A */ rgb!(0x55, 0xC7, 0x53),
    /* 0x2B */ rgb!(0x3C, 0xC9, 0x8C),
    /* 0x2C */ rgb!(0x3E, 0xC2, 0xCD),
    /* 0x2D */ rgb!(0x4E, 0x4E, 0x4E),
    /* 0x2E */ rgb!(0x00, 0x00, 0x00),
    /* 0x2F */ rgb!(0x00, 0x00, 0x00),
    /* 0x30 */ rgb!(0xFE, 0xFE, 0xFF),
    /* 0x31 */ rgb!(0xBC, 0xDF, 0xFF),
    /* 0x32 */ rgb!(0xD1, 0xD8, 0xFF),
    /* 0x33 */ rgb!(0xE8, 0xD1, 0xFF),
    /* 0x34 */ rgb!(0xFB, 0xCD, 0xFD),
    /* 0x35 */ rgb!(0xFF, 0xCC, 0xE5),
    /* 0x36 */ rgb!(0xFF, 0xCF, 0xCA),
    /* 0x37 */ rgb!(0xF8, 0xD5, 0xB4),
    /* 0x38 */ rgb!(0xE4, 0xDC, 0xA8),
    /* 0x39 */ rgb!(0xCC, 0xE3, 0xA9),
    /* 0x3A */ rgb!(0xB9, 0xE8, 0xB8),
    /* 0x3B */ rgb!(0xAE, 0xE8, 0xD0),
    /* 0x3C */ rgb!(0xAF, 0xE5, 0xEA),
    /* 0x3D */ rgb!(0xB6, 0xB6, 0xB6),
    /* 0x3E */ rgb!(0x00, 0x00, 0x00),
    /* 0x3F */ rgb!(0x00, 0x00, 0x00),
];

pub fn lookup(colour_index: u8) -> [f32; 4] {
    let [r, g, b] = COLOUR_TABLE[(colour_index & COLOUR_MASK) as usize];
    [r, g, b, 1.]
}
