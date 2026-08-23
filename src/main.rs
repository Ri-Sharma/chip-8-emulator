use std::env;

use crate::emulator::Emulator;

mod chip8;
mod opcodes;
mod emulator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: chip-8-emulator <rom-path>");
        std::process::exit(1);
    }

    let rom_path: &str = &args[1];
    
    Emulator::new(rom_path).run();
}