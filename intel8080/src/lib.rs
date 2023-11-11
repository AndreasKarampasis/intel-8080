const NUM_REGS: usize = 7;
const MEMORY_SIZE: usize = 65_536;

struct ConditionCodes {
    z: u8,
    s: u8,
    p: u8,
    cy: u8,
    ac: u8,
}

impl ConditionCodes {
    fn new() -> Self {
        Self {
            z: 0,
            s: 0,
            p: 0,
            cy: 0,
            ac: 0,
        }
    }
}
pub struct Intel8080 {
    regs: [u8; NUM_REGS],
    memory: [u8; MEMORY_SIZE],
    pc: u16,
    sp: u16,
    cc: ConditionCodes,
    int_enable: bool,
}

impl Intel8080 {
    pub fn new() -> Self {
        Self {
            regs: [0; NUM_REGS],
            memory: [0; MEMORY_SIZE],
            pc: 0,
            sp: 0,
            cc: ConditionCodes::new(),
            int_enable: false,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        let end = data.len();
        self.memory[..end].copy_from_slice(data);
    }

    fn fetch(&mut self) -> u8 {
        let op = self.memory[self.pc as usize];
        op
    }

    pub fn disassemble(&mut self, op: u8) -> u16 {
        let mut opbytes: u16 = 1;
        match op {
            0x00 => {}
            _ => unimplemented!("Unimplemented opcode: {:#02X}", op),
        }
        opbytes += 2;
        println!("{:02X}", op);
        opbytes
    }

    pub fn tick(&mut self) {
        // Fetch
        let op: u8 = self.fetch();
        // Decode && Execute
        self.pc += self.disassemble(op);
    }
}
