use crate::font;
use crate::lib::*;
use rand::Rng;

const DEBUG: bool = false;

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

    /**
     * (0,0)                  (63,0)
     *  ----------------------->
     * | -----------------------
     * | |                     |
     * | |                     |
     * | |                     |
     * v -----------------------
     * (0,31)                 (63,31)
     */
    pub gfx: [bool; 64 * 32],

    pub delay_timer: u8,

    pub sound_timer: u8,

    timer_cycles: u32,

    stack: [u16; 16],

    sp: u8,

    /**
     * Keyboard layout is as follows:
     * true means pressed, otherwise false.
     *
     * e.g. if keyboard[0xC] == true, then it means that the key
     * corresponding to the button `C` is pressed. (4 is default)
     *
     * |---|---|---|---|
     * | 1 | 2 | 3 | C |
     * |---|---|---|---|
     * | 4 | 5 | 6 | D |
     * |---|---|---|---|
     * | 7 | 8 | 9 | E |
     * |---|---|---|---|
     * | A | 0 | B | F |
     * |---|---|---|---|
     */
    pub keyboard: [bool; 16],

    // should the display be redrawn?
    pub redraw: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Cpu {
            opcode: 0,
            memory: [0; 4096],
            reg: [0; 16],
            index: 0,
            pc: 0,
            gfx: [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            keyboard: [false; 16],
            redraw: false,
            timer_cycles: 0,
        };

        cpu.load_fonts();
        cpu
    }

    /**
     * load font mapping from 0x00 ~ 0x50
     * in this implementation, the fonts are stored as:
     * 0: $00-$04
     * 1: $05-$09
     * 2: $0A-$0E
     * ...
     * F: $4B-$4F
     * http://www.cs.columbia.edu/~sedwards/classes/2016/4840-spring/designs/Chip8.pdf
     */
    fn load_fonts(&mut self) {
        for addr in 0..0x50 {
            self.memory[addr] = font::FONTS[addr];
        }
    }

    // load rom into memory. starts at 0x200
    pub fn load_rom(&mut self, buf: &Vec<u8>) {
        for (index, &c) in buf.iter().enumerate() {
            self.memory[0x200 + index] = c;
        }
        self.pc = 0x200;
    }

    /**
     * update_timers
     * fps: at what frame rate is this emulator running at?
     */
    pub fn update_timers(&mut self, fps: u32) {
        self.timer_cycles += 1;

        // the timers should be updated at 60fps speed.
        // since emulating 60fps shows very laggy gameplay,
        // this emulator allows custom fps (1000fps by default)
        // so use a custom cycle variable to stay true to the original
        // update rate
        if self.timer_cycles == (fps / 60) {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }

            if self.sound_timer > 0 {
                self.sound_timer -= 1;
                // TODO: play sound. to be implemented
            }

            self.timer_cycles = 0;
        }
    }

    pub fn dump_state(&mut self) {
        println!("opcode: {:#04x}", self.opcode);
        println!("Registers:   V0   V1   V2   V3   V4   V5   V6   V7   V8   V9   VA   VB   VC   VD   VE   VF");

        print!("          ");
        for v in self.reg {
            print!(" {:#04x}", v);
        }
        println!();

        println!(
            "I: {:#04x}\tPC: {:#04x}\tSP: {:#04x}",
            self.index, self.pc, self.sp
        );

        println!("Delay: {}\tSound: {}", self.delay_timer, self.sound_timer);

        println!();
    }

    pub fn execute_inst(&mut self) {
        if DEBUG {
            self.dump_state();
        }

        let opcode = (self.memory[self.pc as usize] as u16) << 8
            | self.memory[(self.pc + 1) as usize] as u16;
        self.opcode = opcode;

        let highest_nibble = opcode >> 12;
        let lowest_nibble = opcode & 0x000F;

        let lower_three_nibbles = opcode & 0x0FFF;
        let lower_two_nibbles = opcode & 0x00FF;

        match highest_nibble {
            0x0 => match lower_three_nibbles {
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
            0x8 => match lowest_nibble {
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
            0xE => match lower_two_nibbles {
                0x9E => self.skp_reg(),
                0xA1 => self.sknp_reg(),
                _ => panic!("Invalid Instruction {}", opcode),
            },
            0xF => match lower_two_nibbles {
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
            },
            _ => unimplemented!(),
        }
    }

    // clear the display
    fn cls(&mut self) {
        for i in 0..(64 * 32) {
            self.gfx[i] = false;
        }

        self.redraw = true;
        self.pc += 2;
    }

    // return from subroutine
    fn ret(&mut self) {
        // the stack pointer points to the next empty space.

        // stack must not be empty
        assert!(self.sp != 0);

        // decrement stack pointer
        self.sp -= 1;

        let ret_addr = self.stack[self.sp as usize];
        self.pc = ret_addr;

        self.pc += 2;
    }

    // jump to location nnn
    fn jmp(&mut self) {
        // jump, not a call.
        let addr = self.opcode & 0x0FFF;
        self.pc = addr;
    }

    // call subroutine at nnn
    fn call(&mut self) {
        // store current address
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;

        // jump to given address
        let addr = self.opcode & 0x0FFF;
        self.pc = addr;
    }

    // skip next instruction reg == byte
    fn se_reg_byte(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let kk = self.opcode & 0x00FF;

        let vx = self.reg[x] as u16;

        if vx == kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // skip next instruction reg != byte
    fn sne_reg_byte(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let kk = (self.opcode & 0x00FF) as u8;

        let vx = self.reg[x];

        if vx != kk {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // skip next instruction reg == reg
    fn se_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;

        let vx = self.reg[x];
        let vy = self.reg[y];

        if vx == vy {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // skip next instruction reg != reg
    fn ld_reg_byte(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let kk = (self.opcode & 0x00FF) as u8;
        self.reg[x] = kk;

        self.pc += 2;
    }

    // add byte to register
    fn add_reg_byte(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let kk = (self.opcode & 0x00FF) as u8;
        self.reg[x] = wrap_add(self.reg[x], kk);

        self.pc += 2;
    }

    // load register value to another
    fn ld_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;
        self.reg[x] = self.reg[y];

        self.pc += 2;
    }

    // OR register value with another
    fn or_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;
        self.reg[x] |= self.reg[y];

        self.pc += 2;
    }

    // AND register value with another
    fn and_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;
        self.reg[x] &= self.reg[y];

        self.pc += 2;
    }

    // XOR register value with another
    fn xor_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;
        self.reg[x] ^= self.reg[y];

        self.pc += 2;
    }

    // add register value to another
    fn add_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;

        let vx = self.reg[x];
        let vy = self.reg[y];

        // set carry bit
        if (vx as u16) + (vy as u16) > 0xFF {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }

        self.reg[x as usize] = wrap_add(vx, vy);

        self.pc += 2;
    }

    // subtract register value to another
    fn sub_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;

        let vx = self.reg[x];
        let vy = self.reg[y];

        // NOTE: instruction says set VF to "NOT borrow"
        // watch out for the equal sign
        if vx >= vy {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }

        self.reg[x] = wrap_sub(vx, vy);

        self.pc += 2;
    }

    // store shifted register value to another
    fn shr_reg_reg(&mut self) {
        // NOTE: cowgod's instruction manual doesn't use Vy, not sure why
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let vx = self.reg[x];

        self.reg[0xF] = vx & 0x1;
        self.reg[x] >>= 1;

        self.pc += 2;
    }

    // subn
    fn subn_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;

        let vx = self.reg[x];
        let vy = self.reg[y];

        // NOTE: instruction says set VF to "NOT borrow"
        // watch out for the equal sign
        if vy >= vx {
            self.reg[0xF] = 1;
        } else {
            self.reg[0xF] = 0;
        }

        self.reg[x] = wrap_sub(vy, vx);

        self.pc += 2;
    }

    // shl
    fn shl_reg_reg(&mut self) {
        // NOTE: cowgod's instruction manual doesn't use Vy, not sure why
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let vx = self.reg[x];

        self.reg[0xF] = (vx & 0x80) >> 7;
        self.reg[x] <<= 1;

        self.pc += 2;
    }

    // sne
    fn sne_reg_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;

        let vx = self.reg[x];
        let vy = self.reg[y];

        // NOTE: instruction says set VF to "NOT borrow"
        // watch out for the equal sign
        if vx != vy {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // load value to index register
    fn ld_index_addr(&mut self) {
        let nnn = self.opcode & 0x0FFF;
        self.index = nnn;

        self.pc += 2;
    }

    // jump to location nnn + v0
    fn jmp_rel(&mut self) {
        let nnn = self.opcode & 0x0FFF;
        let v0 = self.reg[0] as u16;

        self.pc = nnn + v0;
    }

    // AND random value with value
    fn rnd_reg_byte(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_byte: u8 = rng.gen_range(0..=255);

        let x = get_nth_nibble(self.opcode, 3);
        let kk = (self.opcode & 0x00FF) as u8;

        self.reg[x as usize] = rand_byte & kk;

        self.pc += 2;
    }

    /**
     * XOR sprites at display and set VF = collision
     */
    fn draw_sprite(&mut self) {
        // this flag should be set if any bit is erased (1 -> 0)
        let mut vf = 0;

        let x = get_nth_nibble(self.opcode, 3) as usize;
        let y = get_nth_nibble(self.opcode, 2) as usize;

        let vx = self.reg[x];
        let vy = self.reg[y];
        let n = get_nth_nibble(self.opcode, 1);

        for i in 0..n {
            let pos_y = wrap_add(vy, i as u8) as u16;
            let sprite_byte = self.memory[(self.index + i as u16) as usize];

            // iterate through bits in sprite_byte
            for j in 0..8 {
                let pos_x = wrap_add(vx, j as u8) as u16;

                let mask = 0x80 >> j;
                let sprite_bit = sprite_byte & mask;

                /* 0 ^ 0 = 0
                 * 0 ^ 1 = 1
                 * 1 ^ 0 = 1
                 * 1 ^ 1 = 0
                 */
                // if sprite_bit is 0, the display won't change. so skip
                if sprite_bit == 0 {
                    continue;
                }

                let point_index = ((64 * pos_y + pos_x + 64 * 32) % (64 * 32)) as usize;

                let display_bit = self.gfx[point_index];

                // if display bit is set, then it will be erased (1 -> 0)
                // Collision!
                if display_bit {
                    vf = 1;
                }

                // since the sprite bit is 1, we know that rhs will be toggled
                self.gfx[point_index] = !self.gfx[point_index];
            }
        }

        self.redraw = true;
        self.reg[0xF] = vf;
        self.pc += 2;
    }

    // skip next inst if key with value of Vx is pressed
    fn skp_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let vx = self.reg[x] as usize;

        if self.keyboard[vx] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // skip next inst if key with value of Vx is not pressed
    fn sknp_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let vx = self.reg[x] as usize;

        if !self.keyboard[vx] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    fn ld_reg_dt(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        self.reg[x] = self.delay_timer;

        self.pc += 2;
    }

    fn ld_reg_key(&mut self) {
        let pressed_key_index = self.keyboard.iter().position(|&x| x);

        match pressed_key_index {
            Some(index) => {
                assert!(index < 16);

                let x = get_nth_nibble(self.opcode, 3) as usize;
                self.reg[x] = index as u8;

                self.pc += 2;
            }
            None => (), // pass until key is pressed
        };
    }

    fn ld_dt_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        self.delay_timer = self.reg[x];

        self.pc += 2;
    }

    fn ld_st_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        self.sound_timer = self.reg[x];

        self.pc += 2;
    }

    fn add_index_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        self.index += self.reg[x] as u16;

        self.pc += 2;
    }

    fn ld_sprite_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let vx = self.reg[x] as u16;
        assert!(vx <= 0xF);

        self.index = vx * 0x5;

        self.pc += 2;
    }

    fn ld_bcd_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3) as usize;
        let vx = self.reg[x];

        let index = self.index as usize;

        self.memory[index] = vx / 100;
        self.memory[index + 1] = (vx / 10) % 10;
        self.memory[index + 2] = vx % 10;

        self.pc += 2;
    }

    fn ld_indirect_reg(&mut self) {
        let x = get_nth_nibble(self.opcode, 3);
        let index = self.index as usize;

        for i in 0..=x {
            self.memory[index + i as usize] = self.reg[i as usize];
        }

        self.pc += 2;
    }

    fn ld_reg_indirect(&mut self) {
        let x = get_nth_nibble(self.opcode, 3);
        let index = self.index as usize;

        for i in 0..=x {
            self.reg[i as usize] = self.memory[index + i as usize];
        }

        self.pc += 2;
    }
}
