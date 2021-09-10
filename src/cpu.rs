use crate::font;

pub struct Cpu {
    // opcode is two bytes long
    opcode: u16,

    // 4KB memory. 0x000 ~ 0x1FF
    // 0x000-0x080: Reserved for Interpreter
    // 0x200-0xFFF: Program ROM and work RAM
    memory: [u8; 4096],

    // 16 8-bit registers
    reg: [u8; 16],

    // I register
    index: u16,

    // program counter
    pc: u16,

    gfx: [u8; 64 * 32],

    delay_timer: u8,

    sound_timer: u8,

    stack: [u16; 16],

    sp: u8,

    keyboard: [u8; 16],
}

impl Cpu {
    fn new() -> Self {
        let mut cpu = Cpu {
            opcode: 0,
            memory: [0; 4096],
            reg: [0; 16],
            index: 0,
            pc: 0,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keyboard: [0; 16],
        };

        cpu.init_fonts();
        cpu
    }

    // initialize font mapping from 0x00 ~ 0x50
    // in this implementation, the fonts are stored as:
    // 0: $00-$04
    // 1: $05-$09
    // 2: $0A-$0E
    // ...
    // F: $4B-$4F
    // http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf
    fn init_fonts(&mut self) {
        for addr in 0..0x50 {
            self.memory[addr] = font::FONTS[addr];
        }
    }

    fn run(&mut self) {
        let opcode = self.opcode;
        let highest_byte = opcode >> 12;
        let lowest_byte = opcode & 0x000F;

        let lower_three_bytes = opcode & 0x0FFF;
        let lower_two_bytes = opcode & 0x00FF;

        match highest_byte {
            0x0 => match lower_three_bytes {
                0x0E0 => self.cls(),
                0x0EE => self.ret(),
                _ => panic!("Invalid Instruction {}", opcode),
            },
            0x1 => self.jmp(),
            0x2 => self.call(),
            0x3 => self.se_reg_byte(),
            0x4 => self.sne_reg_byte(),
            0x5 => self.se_reg_reg(),
            0x6 => self.ld_reg_byte(),
            0x7 => self.add_reg_byte(),
            0x8 => match lowest_byte {
                0x0 => self.ld_reg_reg(),
                0x1 => self.or_reg_reg(),
                0x2 => self.and_reg_reg(),
                0x3 => self.xor_reg_reg(),
                0x4 => self.add_reg_reg(),
                0x5 => self.sub_reg_reg(),
                0x6 => self.shr_reg_reg(),
                0x7 => self.subn_reg_reg(),
                0xE => self.shl_reg_reg(),
                _ => panic!("Invalid Instruction {}", opcode),
            },
            0x9 => self.sne_reg_reg(),
            0xA => self.ld_index_addr(),
            0xB => self.jmp_rel(),
            0xC => self.rnd_reg_byte(),
            0xD => self.draw_sprite(),
            0xE => match lower_two_bytes {
                0x9E => self.skp_reg(),
                0xA1 => self.sknp_reg(),
                _ => panic!("Invalid Instruction {}", opcode),
            },
            0xF => match lower_two_bytes {
                0x07 => self.ld_reg_dt(),
                0x0A => self.ld_reg_key(),
                0x15 => self.ld_dt_reg(),
                0x18 => self.ld_st_reg(),
                0x1E => self.add_index_reg(),
                0x29 => self.ld_sprite_reg(),
                0x33 => self.ld_bcd_reg(),
                0x55 => self.ld_indirect_reg(),
                0x65 => self.ld_reg_indirect(),
                _ => panic!("Invalid Instruction {}", opcode),
            }
            _ => unimplemented!(),
        }
    }

    // clear the display
    fn cls(&mut self) {
        todo!()
    }

    // return from subroutine
    fn ret(&mut self) {
        todo!()
    }

    // jump to location nnn
    fn jmp(&mut self) {
        todo!()
    }

    // call subroutine at nnn
    fn call(&mut self) {
        todo!()
    }

    // skip next instruction reg == byte
    fn se_reg_byte(&mut self) {
        todo!()
    }

    // skip next instruction reg != byte
    fn sne_reg_byte(&mut self) {
        todo!()
    }

    // skip next instruction reg == reg
    fn se_reg_reg(&mut self) {
        todo!()
    }

    // skip next instruction reg != reg
    fn ld_reg_byte(&mut self) {
        todo!()
    }

    // add byte to register
    fn add_reg_byte(&mut self) {
        todo!()
    }

    // load register value to another
    fn ld_reg_reg(&mut self) {
        todo!()
    }

    // OR register value with another
    fn or_reg_reg(&mut self) {
        todo!()
    }

    // AND register value with another
    fn and_reg_reg(&mut self) {
        todo!()
    }

    // XOR register value with another
    fn xor_reg_reg(&mut self) {
        todo!()
    }

    // add register value to another
    fn add_reg_reg(&mut self) {
        todo!()
    }

    // subtract register value to another
    fn sub_reg_reg(&mut self) {
        todo!()
    }

    // store shifted register value to another
    fn shr_reg_reg(&mut self) {
        todo!()
    }

    // subn
    fn subn_reg_reg(&mut self) {
        todo!()
    }

    // shl
    fn shl_reg_reg(&mut self) {
        todo!()
    }

    // sne
    fn sne_reg_reg(&mut self) {
        todo!()
    }

    // load value to index register
    fn ld_index_addr(&mut self) {
        todo!()
    }

    // jump to location nnn + v0
    fn jmp_rel(&mut self) {
        todo!()
    }

    // AND random value with value
    fn rnd_reg_byte(&mut self) {
        todo!()
    }

    fn draw_sprite(&mut self) {
        todo!()
    }

    fn skp_reg(&mut self) {
        todo!()
    }

    fn sknp_reg(&mut self) {
        todo!()
    }

    fn ld_reg_dt(&mut self) {
        todo!()
    }

    fn ld_reg_key(&mut self) {
        todo!()
    }

    fn ld_dt_reg(&mut self) {
        todo!()
    }

    fn ld_st_reg(&mut self) {
        todo!()
    }

    fn add_index_reg(&mut self) {
        todo!()
    }

    fn ld_sprite_reg(&mut self) {
        todo!()
    }

    fn ld_bcd_reg(&mut self) {
        todo!()
    }

    fn ld_indirect_reg(&mut self) {
        todo!()
    }

    fn ld_reg_indirect(&mut self) {
        todo!()
    }
}
