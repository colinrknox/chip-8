/// # CHIP-8 CPU implementation
///
pub struct CPU {
    pub registers: [u8; 0x10],
    pub memory: [u8; 0x1000],
    position_in_memory: usize,
    stack: [u16; 0x10],
    stack_pointer: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            registers: [0; 0x10],
            memory: [0; 0x1000],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
        }
    }
    fn read_opcode(&self) -> u16 {
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        op_byte1 << 8 | op_byte2
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            let nnn = opcode & 0x0FFF;
            let kk = (opcode & 0x00FF) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {
                    return;
                }
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x1, _, _, _) => self.jmp(nnn),
                (0x2, _, _, _) => self.call(nnn),
                (0x3, _, _, _) => {
                    if self.registers[x as usize] == kk {
                        self.position_in_memory += 2;
                    }
                }
                (0x4, _, _, _) => {
                    if self.registers[x as usize] != kk {
                        self.position_in_memory += 2;
                    }
                }
                (0x5, _, _, 0) => {
                    if self.registers[x as usize] == self.registers[y as usize] {
                        self.position_in_memory += 2;
                    }
                }
                (0x6, _, _, _) => self.registers[x as usize] = kk,
                (0x7, _, _, _) => self.registers[x as usize] += kk,
                (0x8, _, _, 0) => self.registers[x as usize] = self.registers[y as usize],
                (0x8, _, _, 1) => {
                    self.registers[x as usize] =
                        self.registers[x as usize] | self.registers[y as usize];
                }
                (0x8, _, _, 2) => {
                    self.registers[x as usize] =
                        self.registers[x as usize] & self.registers[y as usize];
                }
                (0x8, _, _, 3) => {
                    self.registers[x as usize] =
                        self.registers[x as usize] ^ self.registers[y as usize];
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                (0x8, _, _, 0x5) => self.sub_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    pub fn add_xy(&mut self, x: u8, y: u8) {
        self.perform_op(x, y, u8::overflowing_add);
    }

    pub fn sub_xy(&mut self, x: u8, y: u8) {
        self.perform_op(x, y, u8::overflowing_sub);
    }

    fn perform_op<F>(&mut self, x: u8, y: u8, f: F)
    where
        F: Fn(u8, u8) -> (u8, bool),
    {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = f(arg1, arg2);
        self.registers[x as usize] = val;

        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }

    pub fn jmp(&mut self, addr: u16) {
        self.position_in_memory = addr as usize;
    }

    pub fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!");
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    pub fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow!");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }
}
