mod cpu;
mod emu;
mod font;
mod gfx;
mod lib;

use std::env;
use emu::Emulator;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: ./scaters <path-to-rom>");
        std::process::exit(1);
    }

    let rom_file = &args[1];

    let mut emu = Emulator::new()?;
    emu.load_rom(rom_file);
    emu.run_loop()?;
    Ok(())
}
