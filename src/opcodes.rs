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
}
