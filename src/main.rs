mod bit;
mod cpu;
mod emu;
mod font;
mod gfx;
mod input;

use emu::Emulator;

fn main() -> Result<(), String> {
    let mut emu = Emulator::new()?;
    emu.run_loop()?;
    Ok(())
}
