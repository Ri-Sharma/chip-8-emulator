use crate::chip8::Chip8;

mod chip8;

fn main() {
    println!("Starting here <><><><><><><>");
    let mut rom: Vec<u8> = Chip8::load_rom("ROM/IBM Logo.ch8");
    let mut cpu:Chip8 = Chip8::load_from_rom("ROM/IBM Logo.ch8");
    

    for i in rom {
        print!(" {}", i);
    }
    println!();
    println!("Chip8 Loaded!");
}


