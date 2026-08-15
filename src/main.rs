use crate::chip8::Chip8;

mod chip8;
mod opcodes;

fn main() {
    println!("Starting here <><><><><><><>");
    let romPath: &str = "ROM/1-chip8-logo.ch8";
    let mut rom: Vec<u8> = Chip8::load_rom(romPath);
    let mut cpu:Chip8 = Chip8::load_from_rom(romPath);
    

    for i in rom {
        print!(" 0x{:04X}", i);
    }
    cpu.start();
    println!();

    for i in 0..32 {
        for j in 0..64 {
            let idx:usize = i * 64 + j;
            if(cpu.display[idx] == 0) {
                print!("██");
            }
            else {
                print!("  ");
            }
        }
        println!();
    }
    println!();
    println!("Chip8 Loaded!");
}


