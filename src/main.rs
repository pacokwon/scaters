mod cpu;
mod emu;
mod font;
mod gfx;
mod input;
mod lib;

use emu::Emulator;

fn main() -> Result<(), String> {
    let mut emu = Emulator::new()?;
    emu.run_loop()?;
    Ok(())
}
