pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub pc: u16,
    pub status: u8,
    memory: [u8, 0xFFFF]
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            pc: 0,
            status: 0,
        }
    }
    pub fn loader(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.pc = 0x8000
        self.run();
    }

    fn read_u16(&mut self, pos: u16) -> u16 {
        let low = self.mem_read(pos) as u16;
        let high = self.mem_read(pos + 1) as u16;
        return (high << 8) | (low as u16)
    }
    
    fn read(&self, address: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.memory[addr as uszie] = data;
    }


    fn lda(&mut self, val: u8) {
        self.register_a = val;
        self.update_zero_neg_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_neg_flags(self.register_x);
    }

    fn inx(&mut self) {
        if self.register_x == 0xFF {
            self.register_x = 0;
        } 
        else {
            self.register_x += 1;
        }
        self.update_zero_neg_flags(self.register_x);
    }

    fn update_zero_neg_flags(&mut self, res: u8) {
        if res == 0 {
            self.status |= 0b0000_0010;
        }
        else {
            self.status &=  0b1111_1101;
        }
        if res & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        }
        else {
            self.status &= 0b0111_1111;
        }
    }
    pub fn run(&mut self, program: Vec<u8>) {
        loop {
            let instruction = program[self.pc as usize];
            self.pc += 1;
            match instruction {
                //LDA
                0xA9 => {
                    let val = program[self.pc as usize];
                    self.pc += 1;
                    self.lda(val);
                }
                //BRK
                0x00 => {
                    return;
                }
                //TAX
                0xAA => {
                    self.tax();
                }
                //INX
                0xe8 => {
                    self.inx()
                }
                _ => {

                }
            }
        }
    }
}