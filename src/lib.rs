use std::num::Wrapping;
use sdl2::keyboard::Keycode;

/**
 * get_nth_nibble
 *
 * e.g.
 *   get_nth_nibble(0xABCD, 1) == 0xD
 *   get_nth_nibble(0xABCD, 2) == 0xC
 */
pub fn get_nth_nibble(num: u16, n: u8) -> u8 {
    assert!(1 <= n && n <= 4);

    let shift_width = (n - 1) * 4;
    let shifted = num >> shift_width;
    (shifted & 0xF) as u8
}

/**
 * add with overflow / underflow
 */
pub fn wrap_add(a: u8, b: u8) -> u8 {
    let a = Wrapping(a);
    let b = Wrapping(b);
    (a + b).0
}

/**
 * sub with overflow / underflow
 */
pub fn wrap_sub(a: u8, b: u8) -> u8 {
    let a = Wrapping(a);
    let b = Wrapping(b);
    (a - b).0
}

/**
 * A mapping:
 *  |---|---|---|---|       |---|---|---|---|
 *  | 1 | 2 | 3 | 4 |       | 1 | 2 | 3 | C |
 *  |---|---|---|---|       |---|---|---|---|
 *  | Q | W | E | R |       | 4 | 5 | 6 | D |
 *  |---|---|---|---|  -->  |---|---|---|---|
 *  | A | S | D | F |       | 7 | 8 | 9 | E |
 *  |---|---|---|---|       |---|---|---|---|
 *  | Z | X | C | V |       | A | 0 | B | F |
 *  |---|---|---|---|       |---|---|---|---|
 *
 */
pub fn keycode_to_index(keycode: Keycode) -> Option<usize> {
    match keycode {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),
        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),
        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),
        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}
