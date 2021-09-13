use sdl2::keyboard::Keycode;

pub static KEYBOARD_VALUES: [u8; 16] = [
    0x1, 0x2, 0x3, 0xC, 0x4, 0x5, 0x6, 0xD0, 0x7, 0x8, 0x9, 0xE, 0xA, 0x0, 0xB, 0xF,
];

pub fn keycode_to_index(keycode: Keycode) -> Option<usize> {
    match keycode {
        Keycode::Num1 => Some(0),
        Keycode::Num2 => Some(1),
        Keycode::Num3 => Some(2),
        Keycode::Num4 => Some(3),
        Keycode::Q => Some(4),
        Keycode::W => Some(5),
        Keycode::E => Some(6),
        Keycode::R => Some(7),
        Keycode::A => Some(8),
        Keycode::S => Some(9),
        Keycode::D => Some(10),
        Keycode::F => Some(11),
        Keycode::Z => Some(12),
        Keycode::X => Some(13),
        Keycode::C => Some(14),
        Keycode::V => Some(15),
        _ => None,
    }
}

pub fn index_to_keycode(index: usize) -> Option<Keycode> {
    match index {
        0 => Some(Keycode::Num1),
        1 => Some(Keycode::Num2),
        2 => Some(Keycode::Num3),
        3 => Some(Keycode::Num4),
        4 => Some(Keycode::Q),
        5 => Some(Keycode::W),
        6 => Some(Keycode::E),
        7 => Some(Keycode::R),
        8 => Some(Keycode::A),
        9 => Some(Keycode::S),
        10 => Some(Keycode::D),
        11 => Some(Keycode::F),
        12 => Some(Keycode::Z),
        13 => Some(Keycode::X),
        14 => Some(Keycode::C),
        15 => Some(Keycode::V),
        _ => None,
    }
}
