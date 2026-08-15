use std::{fs::File, io::Read};

const FONT: [u8; 80]  = [
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
    pub ram : [u8; 4096],
    pub v : [u8; 16], // registers
    pub idx : u16, // index register
    pub pc : usize, // program counter
    pub stack : [u16; 16],
    pub sp : u8, // stack pointer
    pub display : [u8; 64 * 32],
    pub dt : u8, // delay timer
    pub st : u8, // sound timer
    pub keypad : [bool; 16]
}

impl Chip8 {
    pub fn new (rom : Vec<u8>) -> Self {
        let mut rm: [u8; 4096] = [0; 4096];
        
        // load font
        for i in 0..FONT.len() {
            rm[0x50 + i] = FONT[i];
        }

        // load rom
        for i in 0..rom.len() {
            rm[0x200 + i] = rom[i];
        }


        return Self {
            ram : rm,
            v : [0; 16],
            idx : 0,
            pc : 0x200,
            stack : [0; 16],
            sp : 0,
            display : [0; 64 * 32],
            dt : 0,
            st : 0,
            keypad : [false; 16]
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

    pub fn start(&mut self) -> (){
        self.cycle();
    }

    fn cycle(&mut self) -> () {
        let mut i: i32 = 1000;
        while i > 0  {
            
            let mut opcode: u16 = self.fetch();
            let instruction:u16 = Self::decode(opcode);

            self.execute(instruction, opcode);
            
            i -= 1;
        }
    }

    fn execute(&mut self, instruction: u16, opcode: u16) {
        match instruction {
            0 => {
                match opcode {
                    0x00E0 => self.clear_screen(),
                    _ => print!("Unknown opcode : {:#06X}", opcode)
                }
            },
            1 => self.jump(opcode),
            6 => self.set_register(opcode),
            7 => self.add_to_register( opcode),
            0xA => self.set_index(opcode),
            0xD => self.draw(opcode),
            _ => println!("Unknown opcode : {:#06X}", opcode)
        }
    }

}