use std::{fs::File, io::Read};

const FONT: [u32; 80]  = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Chip8 {
    ram : [u32; 4096],
    v : [u8; 16], // registers
    i : u16, // index register
    pc : u32, // program counter
    stack : [u16; 16],
    sp : u8, // stack pointer
    display : [u8; 64 * 32],
    dt : u8, // delay timer
    st : u8, // sound timer
    keypad : Vec<u8>
}

impl Chip8 {
    pub fn new (rom : Vec<u8>) -> Self {
        let mut rm: [u32; 4096] = [0; 4096];
        
        // load font
        for i in 0..FONT.len() {
            rm[0x50 + i] = FONT[i];
        }

        // load rom
        for i in 0..rom.len() {
            rm[0x200 + i] = rom[i] as u32;
        }


        return Self {
            ram : rm,
            v : [0; 16],
            i : 0,
            pc : 0x200,
            stack : [0; 16],
            sp : 0,
            display : [0; 64 * 32],
            dt : 0,
            st : 0,
            keypad : vec![0; 16]
        }
    }

    pub fn load_from_rom(path : &str) -> Self {
        let rom = Self::load_rom(path);
        return Self::new(rom);
    }


    pub fn load_rom (path : &str) -> Vec<u8> {
        let mut file = File::open(path).expect("Not able to read the file.");

        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer).expect("Something went wrong while reading the file.");

        return buffer;
    }
}