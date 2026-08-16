use crate::chip8::Chip8;

impl Chip8 {
    pub fn fetch (&mut self) -> u16 {
        let pt1:u16 = self.ram[self.pc] as u16;
        let pt2:u16 = self.ram[self.pc + 1] as u16;

        self.pc += 2;

        return (pt1 << 8) | pt2;
    }

    pub fn decode(opcode:u16) -> u16 {
        return (opcode >> 12) & 0xF
    }

    pub fn clear_screen(&mut self) {
        // 00E0
        for i in 0..self.display.len() {
            self.display[i] = 0;
        }
    }

    pub fn jump(&mut self, opcode:u16) -> () {
        // 1NNN
        let pos = opcode & 0x0FFF;
        self.pc = pos as usize;
    }

    pub fn set_register (&mut self, opcode:u16) -> () {
        // 6XNN
        let reg = (opcode >> 8) & 0xF;
        let val = (opcode & 0xFF) as u8;
        self.v[reg as usize] = val;
    }

    pub fn add_to_register (&mut self, opcode:u16) -> () {
        // 7XNN
        let reg = (opcode >> 8) & 0xF;
        let val = (opcode & 0xFF) as u8;
        self.v[reg as usize] += val;
    }

    pub fn set_index(&mut self, opcode:u16) -> () {
        // ANNN
        let val = opcode & 0x0FFF;
        self.idx = val;
    }

    pub fn draw(&mut self, opcode:u16) -> () {
        // DXYN
        let x = (opcode >> 8) & 0xF;
        let y = (opcode >> 4) & 0xF;
        let n = opcode & 0xF;

        let vx = self.v[x as usize] % 64;
        let vy = self.v[y as usize] % 32;
        self.v[0xF] = 0;

        for i in 0..n {
            let sprite = self.ram[(self.idx + i) as usize];
            for j in 0..8 {
                let col = (vx + j) as usize;
                let row = vy as usize + i as usize;

                if row >= 32 || col >= 64  {
                    continue;
                }

                let idx = row * 64 + col; // <<<
                let bit_val = (sprite >> (7 - j)) & 1;

                if self.display[idx] == 1 && bit_val == 1  {
                    self.v[0xF] = 1;
                }
                self.display[idx] = self.display[idx] ^ bit_val;
            }
        }

    }

    pub fn call_subroutine(&mut self, opcode:u16) {
        // 2NNN
        let pos = opcode & 0x0FFF;

        self.stack[self.sp as usize] = self.pc as u16;
        self.sp = self.sp + 1;

        self.pc = pos as usize;
    }

    pub fn return_from_subroutine(&mut self) {
        // 00EE
        self.pc = self.stack[self.sp as usize] as usize;
        self.sp = self.sp - 1;
    }

    pub fn skip_if_eq(&mut self, opcode:u16){
        // 3XNN
        let x = (opcode >> 8) & 0xF;
        let nn = opcode & 0xFF;

        if(self.v[x as usize] == nn as u8) {
            self.pc = self.pc + 2;
        }
    }

    pub fn skip_if_neq(&mut self, opcode:u16){
        // 4XNN
        let x = (opcode >> 8) & 0xF;
        let nn = opcode & 0xFF;

        if self.v[x as usize] != nn as u8 {
            self.pc = self.pc + 2;
        }
    }

    pub fn skip_if_x_eq_y(&mut self, opcode:u16){
        // 5XY0
        let x = (opcode >> 8) & 0xF;
        let y = (opcode >> 4) & 0xF;

        if self.v[x as usize] == self.v[y as usize] {
            self.pc = self.pc + 2;
        }
    }

    pub fn skip_if_x_neq_y(&mut self, opcode:u16){
        // 9XY0
        let x = (opcode >> 8) & 0xF;
        let y = (opcode >> 4) & 0xF;

        if self.v[x as usize] != self.v[y as usize] {
            self.pc = self.pc + 2;
        }
    }

    pub fn set(&mut self, opcode: u16) {
        // 8XY0
        let x = (opcode >> 8) & 0xF;
        let y = (opcode >> 4) & 0xF;

        self.v[x as usize] = self.v[y as usize];
    }

    pub fn binary_or(&mut self, opcode: u16) {
        // 8XY1
        let x: usize = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] |= self.v[y];
        
    }

    pub fn binary_and(&mut self, opcode: u16) {
        // 8XY2
        let x: usize = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] &= self.v[y];
        
    }

    pub fn logical_xor(&mut self, opcode: u16) {
        // 8XY3
        let x: usize = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] ^= self.v[y];
        
    }

    pub fn add(&mut self, opcode: u16) {
        // 8XY4
        let x: usize = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        let val_x = self.v[x] as u16;
        let val_y = self.v[y] as u16;

        if val_x + val_y > 0xFF {
            self.v[0xF] = 1;
        }
        else {
            self.v[0xF] = 0;
        }

        self.v[x] += self.v[y];
        
    }

    pub fn subtract(&mut self, opcode: u16) {
        // 8XY5
        let x  = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] = self.v[x] - self.v[y];
    }

    pub fn subtract_from(&mut self, opcode: u16) {
        // 8XY7
        let x = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] = self.v[y] - self.v[x];
    }

    pub fn shift_right(&mut self, opcode: u16) {
        // 8XY6
        let x = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] = self.v[y];

        self.v[0xF] = self.v[x] & 1;

        self.v[x] >>= 1;
    }

    pub fn shift_left(&mut self, opcode: u16) {
        // 8XYE
        let x = (opcode as usize >> 8) & 0xF;
        let y = (opcode as usize >> 4) & 0xF;

        self.v[x] = self.v[y];

        self.v[0xF] = (self.v[x] >> 7) & 1;

        self.v[x] <<= 1;
    }

    pub fn set_from_delay_timer(&mut self, opcode: u16) {
        // FX07
        let x = (opcode as usize >> 8) & 0xF;

        self.v[x] = self.dt;
    }

    pub fn set_delay_timer(&mut self, opcode: u16) {
        // FX15
        let x = (opcode as usize >> 8) & 0xF;

        self.dt = self.v[x];
    }

    pub fn set_sound_timer(&mut self, opcode: u16) {
        // FX18
        let x = (opcode as usize >> 8) & 0xF;

        self.st = self.v[x];
    }

    pub fn add_to_index(&mut self, opcode: u16) {
        // FX1E
        let x = (opcode as usize >> 8) & 0xF;

        if self.idx + self.v[x] as u16 > 0xFFFF { // <<! how to signify overflow with only 16 bits variable type?
            self.v[0xF] = 1;
        }

        self.idx += self.v[x] as u16;
    }

    pub fn get_key(&mut self, opcode: u16) {
        // FX0A
        let x = (opcode as usize >> 8) & 0xF;

        // Keyboard input
    }

    pub fn font_character(&mut self, opcode: u16) {
        // FX29
        let x = (opcode as usize >> 8) & 0xF;

        self.idx = 0x50 + (5 * self.v[x] as u16) // << confirm this.
    }

    pub fn binary_coded_decimal_conversion(&mut self, opcode: u16) {
        // FX33 
        let x = (opcode as usize >> 8) & 0xF;
        let mut x_val = self.v[x];

        let mut i:u8 = 2;

        while i >= 0 {
            let dig = x_val % 10;
            self.ram[self.idx as usize + i as usize] = dig;

            x_val /= 10;
            i -= 1;
        }

        // << confirm this as-well
    }

    pub fn store_memory(&mut self, opcode: u16) {
        // FX55
        let x = (opcode as usize >> 8) & 0xF;

        for i in 0..=x {
            self.ram[i + self.idx as usize] = self.v[i]
        }
    }

    pub fn load_memory(&mut self, opcode: u16) {
        // FX65 
        let x = (opcode as usize >> 8) & 0xF;

        for i in 0..=x {
            self.v[i] = self.ram[i + self.idx as usize];
        }
    }

}
