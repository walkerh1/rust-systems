// CHIP-8 CPU emulator

pub struct CPU {
    pub registers: [u8; 16],
    pub memory: [u8; 0x1000],
    position_in_memory: usize,
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >>  8) as u8;
            let y = ((opcode & 0x00F0) >>  4) as u8;
            let d = ((opcode & 0x000F) >>  0) as u8;

            let nnn = opcode & 0x0FFF;

            match (c, x, y, d) {
                (  0,   0,   0,   0) => break,
                (  0,   0, 0xE, 0xE) => self.ret(),
                (0x2,   _,   _,   _) => self.call(nnn),
                (0x8,   _,   _, 0x4) => self.add_xy(x, y),
                _ => todo!()
            }
        }
    }

    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p+1] as u16;
        
        op_byte1 << 8 | op_byte2
    }

    fn call(&mut self, nnn: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = nnn as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let addr = self.stack[self.stack_pointer];
        self.position_in_memory = addr as usize;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2);
        
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}
