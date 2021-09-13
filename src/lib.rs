use std::num::Wrapping;

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
