use crate::emulator::Emulator;

mod chip8;
mod opcodes;
mod emulator;

fn main() {
    let rom_path: &str = "";
    Emulator::new(rom_path).run();
}