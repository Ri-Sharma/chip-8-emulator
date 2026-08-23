use std::{fs::{self}, io::Error};

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

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Chip8 {
    pub ram : [u8; 4096],
    pub v : [u8; 16], // registers
    pub idx : u16, // index register
    pub pc : usize, // program counter
    pub stack : [u16; 16],
    pub sp : u8, // stack pointer
    pub display : [u8; WIDTH * HEIGHT],
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
            display : [0; WIDTH * HEIGHT],
            dt : 0,
            st : 0,
            keypad : [false; 16]
        }
    }

    pub fn load_from_rom(path : &str) -> Self {
        match Self::load_rom(path) {
            Ok(rom)  => Self::new(rom),
            Err(e) => panic!("Not able to read the file: {}. \n{}", path, e),
        }
    }

    fn load_rom (path : &str) -> Result<Vec<u8>, Error> {
        fs::read(path)
    }

    pub fn tick(&mut self) -> () { // Here a tick shows a complete fetch-decode-execute cycle.
         // fetch -- execute -- decode
        let opcode = self.fetch();
        let instruction = Self::decode(opcode);
        self.execute(instruction, opcode);

    }

    fn execute(&mut self, instruction: u16, opcode: u16) {
        match instruction {
            0 => {
                match opcode {
                    0x00E0 => self.clear_screen(),
                    0x00EE => self.return_from_subroutine(),
                    _ => eprintln!("Unknown opcode : {:#04X}", opcode)
                }
            },
            0x1 => self.jump(opcode),
            0x2 => self.call_subroutine(opcode),
            0x3 => self.skip_if_eq(opcode),
            0x4 => self.skip_if_neq(opcode),
            0x5 => self.skip_if_x_eq_y(opcode),
            0x6 => self.set_register(opcode),
            0x7 => self.add_to_register( opcode),
            0x8 => {
                match opcode & 0xF {
                    0x0 => self.set(opcode),
                    0x1 => self.binary_or(opcode),
                    0x2 => self.binary_and(opcode),
                    0x3 => self.logical_xor(opcode),
                    0x4 => self.add(opcode),
                    0x5 => self.subtract(opcode),
                    0x6 => self.shift_right(opcode),
                    0x7 => self.subtract_from(opcode),
                    0xE => self.shift_left(opcode),
                    _ => eprintln!("Unknown opcode : {:#04X}", opcode)
                }
            }
            0x9 => self.skip_if_x_neq_y(opcode),
            0xA => self.set_index(opcode),
            0xD => self.draw(opcode),
            0xE => {
                match opcode & 0xFF {
                    0x9E => self.skip_if_pressed(opcode),
                    0xA1 => self.skip_if_not_pressed(opcode),
                    _ => eprintln!("Unknown opcode : {:#04X}", opcode)
                }
            }
            0xF => {
                match opcode & 0xFF {
                    0x07 => self.set_from_delay_timer(opcode),
                    0x15 => self.set_delay_timer(opcode),
                    0x18 => self.set_sound_timer(opcode),
                    0x1E => self.add_to_index(opcode),
                    0x0A => self.get_key(opcode),
                    0x29 => self.font_character(opcode),
                    0x33 => self.binary_coded_decimal_conversion(opcode),
                    0x55 => self.store_memory(opcode),
                    0x65 => self.load_memory(opcode),
                     _ => eprintln!("Unknown opcode : {:#04X}", opcode)
                }
            }
            _ => eprintln!("Unknown opcode : {:#04X}", opcode)
        }
    }
}