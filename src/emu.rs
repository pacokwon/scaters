use crate::cpu::Cpu;
use crate::gfx::Graphics;
use crate::lib;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct Emulator {
    cpu: Cpu,
    graphics: Graphics,
    fps: u32,
}

impl Emulator {
    pub fn new() -> Result<Self, String> {
        let cpu = Cpu::new();
        let graphics = Graphics::new()?;

        Ok(Emulator {
            cpu,
            graphics,
            fps: 1000, // 1000 fps by default
        })
    }

    pub fn load_rom(&mut self, filename: &str) {
        let buffer = std::fs::read(filename).unwrap();
        self.cpu.load_rom(&buffer);
    }

    /**
     * Main loop of emulator
     *
     * 1. listen to & handle events
     * 2. run opcode
     * 3. update screen
     * 4. update timers
     */
    pub fn run_loop(&mut self) -> Result<(), String> {
        let ctx = &mut self.graphics.ctx;
        let mut event_pump = ctx.event_pump()?;

        'running: loop {
            // 1. listen to & handle events
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        ..
                    } => self.handle_keydown(keycode),
                    Event::KeyUp {
                        keycode: Some(keycode),
                        ..
                    } => self.handle_keyup(keycode),
                    _ => (),
                }
            }

            // 2. run opcode
            self.cpu.execute_inst();

            // 3. update screen
            if self.cpu.redraw {
                self.cpu.redraw = false;
                self.graphics.draw(&self.cpu.gfx)?;
            }

            // 4. update timers
            self.cpu.update_timers(self.fps);

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / self.fps));
        }

        Ok(())
    }

    fn handle_keydown(&mut self, keycode: Keycode) {
        if let Some(index) = lib::keycode_to_index(keycode) {
            self.cpu.keyboard[index] = true;
        }
    }

    fn handle_keyup(&mut self, keycode: Keycode) {
        if let Some(index) = lib::keycode_to_index(keycode) {
            self.cpu.keyboard[index as usize] = false;
        }
    }
}
