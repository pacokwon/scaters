use crate::cpu::Cpu;
use crate::graphics::Graphics;
use sdl2::event::Event;
use std::time::Duration;

pub struct Emulator {
    cpu: Cpu,
    graphics: Graphics,
}

impl Emulator {
    pub fn new() -> Result<Self, String> {
        let cpu = Cpu::new();
        let graphics = Graphics::new()?;

        Ok(Emulator { cpu, graphics })
    }

    pub fn run_loop(&mut self) -> Result<(), String> {
        let ctx = &mut self.graphics.ctx;
        let canvas = &mut self.graphics.canvas;

        let mut event_pump = ctx.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }

            canvas.clear();
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30)); // 30fps
        }

        Ok(())
    }
}
