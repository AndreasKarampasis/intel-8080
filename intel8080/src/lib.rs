const MEMORY_SIZE: usize = 65_536;
// I decided to use an array of 8 registers so that I can get the specified
// register directly from the opcode byte.
// So I have to make sure that the register[M_REF] is never used as a register.
const REGISTER_NUM: usize = 8;
const REG_B: usize = 0x00; // 0b0000_0000
const REG_C: usize = 0x01; // 0b0000_0001
const REG_D: usize = 0x02; // 0b0000_0010
const REG_E: usize = 0x03; // 0b0000_0011
const REG_H: usize = 0x04; // 0b0000_0100
const REG_L: usize = 0x05; // 0b0000_0101
const REG_A: usize = 0x07; // 0b0000_0111
const M_REF: u8 = 0x06; // 0b0000_0110

struct ConditionCodes {
    s: u8,  // Set if the result of an operation is zero.
    z: u8,  // Set if the MS bit of the result is 1, indicating a negative number.
    p: u8,  // Parity flag
    cy: u8, // Carry flag
    ac: u8, // Auxiliary carry flag, (space invaders doesn't use it)
}
pub struct Intel8080 {
    registers: [u8; REGISTER_NUM],
    memory: [u8; MEMORY_SIZE],
    pc: u16,
    sp: u16,
    cc: ConditionCodes,
    interrupts_enable: bool,
}

impl ConditionCodes {
    fn new() -> Self {
        Self {
            s: 0,
            z: 0,
            p: 0,
            cy: 0,
            ac: 0,
        }
    }
}

// Utility function to calculate parity
fn is_parity_even(byte: u8) -> bool {
    let mut count = 0;
    for i in 0..8 {
        if (byte & (1 << i)) != 0 {
            count += 1;
        }
    }
    (count % 2) == 0
}

impl Intel8080 {
    pub fn new() -> Self {
        Self {
            registers: [0; REGISTER_NUM],
            memory: [0; MEMORY_SIZE],
            pc: 0,
            sp: 0,
            cc: ConditionCodes::new(),
            interrupts_enable: false,
        }
    }

    pub fn print_state(&self) {
        println!("opcode: {:#04X}", self.memory[self.pc as usize]);
        println!("        CPU Misc. Field State");
        println!("-------------------------------------------");
        println!("FIELD |DEC\t|HEX\t|BIN               |");
        println!("-------------------------------------------|");
        println!("PC    |{}\t|{:#06X}\t|{:#018b}|", self.pc, self.pc, self.pc);
        println!("SP    |{}\t|{:#06X}\t|{:#018b}|", self.sp, self.sp, self.sp);
        println!("-------------------------------------------|");
        println!("                                           |");
        println!("        CPU Register State                 |");
        println!("-------------------------------------------|");
        println!("REGISTER |DEC\t|HEX\t|BIN               |");
        println!("-------------------------------------------|");
        println!(
            "A        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_A], self.registers[REG_A], self.registers[REG_A]
        );
        println!(
            "B        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_B], self.registers[REG_B], self.registers[REG_B]
        );
        println!(
            "C        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_C], self.registers[REG_C], self.registers[REG_C]
        );
        println!(
            "D        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_D], self.registers[REG_D], self.registers[REG_D]
        );
        println!(
            "E        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_E], self.registers[REG_E], self.registers[REG_E]
        );
        println!(
            "H        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_H], self.registers[REG_H], self.registers[REG_H]
        );
        println!(
            "L        |{}\t|{:#04X}\t|{:#010b}        |",
            self.registers[REG_L], self.registers[REG_L], self.registers[REG_L]
        );
        println!("-------------------------------------------|");
        println!("                                           |");
        println!("        CPU FLAG State                     |");
        println!("-------------------------------------------|");
        println!("        FLAG         |       VALUE         |");
        println!("-------------------------------------------|");
        println!("  CARRY              | {:#04X}\t           |", self.cc.cy);
        println!("  PARITY             | {:#04X}\t           |", self.cc.p);
        println!("  AUX-CARRY          | {:#04X}\t           |", self.cc.ac);
        println!("  ZERO               | {:#04X}\t           |", self.cc.z);
        println!("  SIGN               | {:#04X}\t           |", self.cc.s);
        println!("-------------------------------------------\n\n");
    }

    // TODO: This is just for testing purposes
    pub fn get_pc(&mut self) -> u16 {
        self.pc
    }

    pub fn load(&mut self, data: &[u8]) {
        let end = data.len();
        self.memory[..end].copy_from_slice(data);
    }

    fn fetch(&mut self) -> u8 {
        let op = self.memory[self.pc as usize];
        op
    }

    pub fn tick(&mut self) {
        // Fetch
        let op: u8 = self.fetch();
        // Decode && Execute
        self.execute(op);
    }

    // Update Zero, Sign, and Parity flags based on the contents of a register
    // Carry Flag (CY) and Auxiliary Carry Flag (AC) are not updated here
    // They are usually updated after specific arithmetic or logical operations
    fn update_flags(&mut self, byte: u8) {
        // Update zero flag
        self.cc.z = if byte == 0 { 1 } else { 0 };
        // Sign flag
        self.cc.s = if (byte & 0b1000_0000) != 0 { 1 } else { 0 };
        // Parity Flag
        self.cc.p = if is_parity_even(byte) { 1 } else { 0 };
    }

    // TODO: use array of function pointers for better visibility
    pub fn execute(&mut self, op: u8) {
        match op {
            0x00 => self.pc += 1,
            0x01 => self.lxi(),
            0x02 => unimplemented!("Error: Unimplemented opcode."),
            0x03 => unimplemented!("Error: Unimplemented opcode."),
            0x04 => unimplemented!("Error: Unimplemented opcode."),
            0x05 => self.dcr(),
            0x06 => self.mvi(),
            0x07 => unimplemented!("Error: Unimplemented opcode."),
            0x08 => unimplemented!("Error: Unimplemented opcode."),
            0x09 => todo!("0x09 dad B"),
            0x0a => unimplemented!("Error: Unimplemented opcode."),
            0x0b => unimplemented!("Error: Unimplemented opcode."),
            0x0c => unimplemented!("Error: Unimplemented opcode."),
            0x0d => self.dcr(),
            0x0e => self.mvi(),
            0x0f => todo!("0x0f RRC"),

            0x10 => unimplemented!("Error: Unimplemented opcode."),
            0x11 => self.lxi(),
            0x12 => unimplemented!("Error: Unimplemented opcode."),
            0x13 => self.inx(),
            0x14 => unimplemented!("Error: Unimplemented opcode."),
            0x15 => unimplemented!("Error: Unimplemented opcode."),
            0x16 => unimplemented!("Error: Unimplemented opcode."),
            0x17 => unimplemented!("Error: Unimplemented opcode."),
            0x18 => unimplemented!("Error: Unimplemented opcode."),
            0x19 => todo!("0x19 dad d"),
            0x1a => self.ldax(),
            0x1b => unimplemented!("Error: Unimplemented opcode."),
            0x1c => unimplemented!("Error: Unimplemented opcode."),
            0x1d => unimplemented!("Error: Unimplemented opcode."),
            0x1e => unimplemented!("Error: Unimplemented opcode."),
            0x1f => unimplemented!("Error: Unimplemented opcode."),

            0x20 => unimplemented!("Error: Unimplemented opcode."),
            0x21 => self.lxi(),
            0x22 => unimplemented!("Error: Unimplemented opcode."),
            0x23 => self.inx(),
            0x24 => unimplemented!("Error: Unimplemented opcode."),
            0x25 => unimplemented!("Error: Unimplemented opcode."),
            0x26 => self.mvi(),
            0x27 => unimplemented!("Error: Unimplemented opcode."),
            0x28 => unimplemented!("Error: Unimplemented opcode."),
            0x29 => unimplemented!("Error: Unimplemented opcode."),
            0x2a => unimplemented!("Error: Unimplemented opcode."),
            0x2b => unimplemented!("Error: Unimplemented opcode."),
            0x2c => unimplemented!("Error: Unimplemented opcode."),
            0x2d => unimplemented!("Error: Unimplemented opcode."),
            0x2e => unimplemented!("Error: Unimplemented opcode."),
            0x2f => unimplemented!("Error: Unimplemented opcode."),

            0x30 => unimplemented!("Error: Unimplemented opcode."),
            0x31 => self.lxi(),
            0x32 => unimplemented!("Error: Unimplemented opcode."),
            0x33 => unimplemented!("Error: Unimplemented opcode."),
            0x34 => unimplemented!("Error: Unimplemented opcode."),
            0x35 => unimplemented!("Error: Unimplemented opcode."),
            0x36 => self.mvi(),
            0x37 => unimplemented!("Error: Unimplemented opcode."),
            0x38 => unimplemented!("Error: Unimplemented opcode."),
            0x39 => unimplemented!("Error: Unimplemented opcode."),
            0x3a => unimplemented!("Error: Unimplemented opcode."),
            0x3b => unimplemented!("Error: Unimplemented opcode."),
            0x3c => unimplemented!("Error: Unimplemented opcode."),
            0x3d => unimplemented!("Error: Unimplemented opcode."),
            0x3e => self.mvi(),
            0x3f => unimplemented!("Error: Unimplemented opcode."),

            0x40 => unimplemented!("Error: Unimplemented opcode."),
            0x41 => unimplemented!("Error: Unimplemented opcode."),
            0x42 => unimplemented!("Error: Unimplemented opcode."),
            0x43 => unimplemented!("Error: Unimplemented opcode."),
            0x44 => unimplemented!("Error: Unimplemented opcode."),
            0x45 => unimplemented!("Error: Unimplemented opcode."),
            0x46 => unimplemented!("Error: Unimplemented opcode."),
            0x47 => unimplemented!("Error: Unimplemented opcode."),
            0x48 => unimplemented!("Error: Unimplemented opcode."),
            0x49 => unimplemented!("Error: Unimplemented opcode."),
            0x4a => unimplemented!("Error: Unimplemented opcode."),
            0x4b => unimplemented!("Error: Unimplemented opcode."),
            0x4c => unimplemented!("Error: Unimplemented opcode."),
            0x4d => unimplemented!("Error: Unimplemented opcode."),
            0x4e => unimplemented!("Error: Unimplemented opcode."),
            0x4f => unimplemented!("Error: Unimplemented opcode."),

            0x50 => unimplemented!("Error: Unimplemented opcode."),
            0x51 => unimplemented!("Error: Unimplemented opcode."),
            0x52 => unimplemented!("Error: Unimplemented opcode."),
            0x53 => unimplemented!("Error: Unimplemented opcode."),
            0x54 => unimplemented!("Error: Unimplemented opcode."),
            0x55 => unimplemented!("Error: Unimplemented opcode."),
            0x56 => unimplemented!("Error: Unimplemented opcode."),
            0x57 => unimplemented!("Error: Unimplemented opcode."),
            0x58 => unimplemented!("Error: Unimplemented opcode."),
            0x59 => unimplemented!("Error: Unimplemented opcode."),
            0x5a => unimplemented!("Error: Unimplemented opcode."),
            0x5b => unimplemented!("Error: Unimplemented opcode."),
            0x5c => unimplemented!("Error: Unimplemented opcode."),
            0x5d => unimplemented!("Error: Unimplemented opcode."),
            0x5e => unimplemented!("Error: Unimplemented opcode."),
            0x5f => unimplemented!("Error: Unimplemented opcode."),

            0x60 => unimplemented!("Error: Unimplemented opcode."),
            0x61 => unimplemented!("Error: Unimplemented opcode."),
            0x62 => unimplemented!("Error: Unimplemented opcode."),
            0x63 => unimplemented!("Error: Unimplemented opcode."),
            0x64 => unimplemented!("Error: Unimplemented opcode."),
            0x65 => unimplemented!("Error: Unimplemented opcode."),
            0x66 => unimplemented!("Error: Unimplemented opcode."),
            0x67 => unimplemented!("Error: Unimplemented opcode."),
            0x68 => unimplemented!("Error: Unimplemented opcode."),
            0x69 => unimplemented!("Error: Unimplemented opcode."),
            0x6a => unimplemented!("Error: Unimplemented opcode."),
            0x6b => unimplemented!("Error: Unimplemented opcode."),
            0x6c => unimplemented!("Error: Unimplemented opcode."),
            0x6d => unimplemented!("Error: Unimplemented opcode."),
            0x6e => unimplemented!("Error: Unimplemented opcode."),
            0x6f => unimplemented!("Error: Unimplemented opcode."),

            0x70 => unimplemented!("Error: Unimplemented opcode."),
            0x71 => unimplemented!("Error: Unimplemented opcode."),
            0x72 => unimplemented!("Error: Unimplemented opcode."),
            0x73 => unimplemented!("Error: Unimplemented opcode."),
            0x74 => unimplemented!("Error: Unimplemented opcode."),
            0x75 => unimplemented!("Error: Unimplemented opcode."),
            0x76 => unimplemented!("Error: Unimplemented opcode."),
            0x77 => self.mov(),
            0x78 => unimplemented!("Error: Unimplemented opcode."),
            0x79 => unimplemented!("Error: Unimplemented opcode."),
            0x7a => unimplemented!("Error: Unimplemented opcode."),
            0x7b => unimplemented!("Error: Unimplemented opcode."),
            0x7c => unimplemented!("Error: Unimplemented opcode."),
            0x7d => unimplemented!("Error: Unimplemented opcode."),
            0x7e => unimplemented!("Error: Unimplemented opcode."),
            0x7f => unimplemented!("Error: Unimplemented opcode."),

            0x80 => unimplemented!("Error: Unimplemented opcode."),
            0x81 => unimplemented!("Error: Unimplemented opcode."),
            0x82 => unimplemented!("Error: Unimplemented opcode."),
            0x83 => unimplemented!("Error: Unimplemented opcode."),
            0x84 => unimplemented!("Error: Unimplemented opcode."),
            0x85 => unimplemented!("Error: Unimplemented opcode."),
            0x86 => unimplemented!("Error: Unimplemented opcode."),
            0x87 => unimplemented!("Error: Unimplemented opcode."),
            0x88 => unimplemented!("Error: Unimplemented opcode."),
            0x89 => unimplemented!("Error: Unimplemented opcode."),
            0x8a => unimplemented!("Error: Unimplemented opcode."),
            0x8b => unimplemented!("Error: Unimplemented opcode."),
            0x8c => unimplemented!("Error: Unimplemented opcode."),
            0x8d => unimplemented!("Error: Unimplemented opcode."),
            0x8e => unimplemented!("Error: Unimplemented opcode."),
            0x8f => unimplemented!("Error: Unimplemented opcode."),

            0x90 => unimplemented!("Error: Unimplemented opcode."),
            0x91 => unimplemented!("Error: Unimplemented opcode."),
            0x92 => unimplemented!("Error: Unimplemented opcode."),
            0x93 => unimplemented!("Error: Unimplemented opcode."),
            0x94 => unimplemented!("Error: Unimplemented opcode."),
            0x95 => unimplemented!("Error: Unimplemented opcode."),
            0x96 => unimplemented!("Error: Unimplemented opcode."),
            0x97 => unimplemented!("Error: Unimplemented opcode."),
            0x98 => unimplemented!("Error: Unimplemented opcode."),
            0x99 => unimplemented!("Error: Unimplemented opcode."),
            0x9a => unimplemented!("Error: Unimplemented opcode."),
            0x9b => unimplemented!("Error: Unimplemented opcode."),
            0x9c => unimplemented!("Error: Unimplemented opcode."),
            0x9d => unimplemented!("Error: Unimplemented opcode."),
            0x9e => unimplemented!("Error: Unimplemented opcode."),
            0x9f => unimplemented!("Error: Unimplemented opcode."),

            0xa0 => unimplemented!("Error: Unimplemented opcode."),
            0xa1 => unimplemented!("Error: Unimplemented opcode."),
            0xa2 => unimplemented!("Error: Unimplemented opcode."),
            0xa3 => unimplemented!("Error: Unimplemented opcode."),
            0xa4 => unimplemented!("Error: Unimplemented opcode."),
            0xa5 => unimplemented!("Error: Unimplemented opcode."),
            0xa6 => unimplemented!("Error: Unimplemented opcode."),
            0xa7 => unimplemented!("Error: Unimplemented opcode."),
            0xa8 => unimplemented!("Error: Unimplemented opcode."),
            0xa9 => unimplemented!("Error: Unimplemented opcode."),
            0xaa => unimplemented!("Error: Unimplemented opcode."),
            0xab => unimplemented!("Error: Unimplemented opcode."),
            0xac => unimplemented!("Error: Unimplemented opcode."),
            0xad => unimplemented!("Error: Unimplemented opcode."),
            0xae => unimplemented!("Error: Unimplemented opcode."),
            0xaf => unimplemented!("Error: Unimplemented opcode."),

            0xb0 => unimplemented!("Error: Unimplemented opcode."),
            0xb1 => unimplemented!("Error: Unimplemented opcode."),
            0xb2 => unimplemented!("Error: Unimplemented opcode."),
            0xb3 => unimplemented!("Error: Unimplemented opcode."),
            0xb4 => unimplemented!("Error: Unimplemented opcode."),
            0xb5 => unimplemented!("Error: Unimplemented opcode."),
            0xb6 => unimplemented!("Error: Unimplemented opcode."),
            0xb7 => unimplemented!("Error: Unimplemented opcode."),
            0xb8 => unimplemented!("Error: Unimplemented opcode."),
            0xb9 => unimplemented!("Error: Unimplemented opcode."),
            0xba => unimplemented!("Error: Unimplemented opcode."),
            0xbb => unimplemented!("Error: Unimplemented opcode."),
            0xbc => unimplemented!("Error: Unimplemented opcode."),
            0xbd => unimplemented!("Error: Unimplemented opcode."),
            0xbe => unimplemented!("Error: Unimplemented opcode."),
            0xbf => unimplemented!("Error: Unimplemented opcode."),

            0xc0 => unimplemented!("Error: Unimplemented opcode."),
            0xc1 => unimplemented!("Error: Unimplemented opcode."),
            0xc2 => self.jnz(),
            0xc3 => self.jmp(),
            0xc4 => unimplemented!("Error: Unimplemented opcode."),
            0xc5 => unimplemented!("Error: Unimplemented opcode."),
            0xc6 => unimplemented!("Error: Unimplemented opcode."),
            0xc7 => unimplemented!("Error: Unimplemented opcode."),
            0xc8 => unimplemented!("Error: Unimplemented opcode."),
            0xc9 => unimplemented!("Error: Unimplemented opcode."),
            0xca => unimplemented!("Error: Unimplemented opcode."),
            0xcb => unimplemented!("Error: Unimplemented opcode."),
            0xcc => unimplemented!("Error: Unimplemented opcode."),
            0xcd => self.call(),
            0xce => unimplemented!("Error: Unimplemented opcode."),
            0xcf => unimplemented!("Error: Unimplemented opcode."),

            0xd0 => unimplemented!("Error: Unimplemented opcode."),
            0xd1 => unimplemented!("Error: Unimplemented opcode."),
            0xd2 => unimplemented!("Error: Unimplemented opcode."),
            0xd3 => unimplemented!("Error: Unimplemented opcode."),
            0xd4 => unimplemented!("Error: Unimplemented opcode."),
            0xd5 => unimplemented!("Error: Unimplemented opcode."),
            0xd6 => unimplemented!("Error: Unimplemented opcode."),
            0xd7 => unimplemented!("Error: Unimplemented opcode."),
            0xd8 => unimplemented!("Error: Unimplemented opcode."),
            0xd9 => unimplemented!("Error: Unimplemented opcode."),
            0xda => unimplemented!("Error: Unimplemented opcode."),
            0xdb => unimplemented!("Error: Unimplemented opcode."),
            0xdc => unimplemented!("Error: Unimplemented opcode."),
            0xdd => unimplemented!("Error: Unimplemented opcode."),
            0xde => unimplemented!("Error: Unimplemented opcode."),
            0xdf => unimplemented!("Error: Unimplemented opcode."),

            0xe0 => unimplemented!("Error: Unimplemented opcode."),
            0xe1 => unimplemented!("Error: Unimplemented opcode."),
            0xe2 => unimplemented!("Error: Unimplemented opcode."),
            0xe3 => unimplemented!("Error: Unimplemented opcode."),
            0xe4 => unimplemented!("Error: Unimplemented opcode."),
            0xe5 => unimplemented!("Error: Unimplemented opcode."),
            0xe6 => unimplemented!("Error: Unimplemented opcode."),
            0xe7 => unimplemented!("Error: Unimplemented opcode."),
            0xe8 => unimplemented!("Error: Unimplemented opcode."),
            0xe9 => unimplemented!("Error: Unimplemented opcode."),
            0xea => unimplemented!("Error: Unimplemented opcode."),
            0xeb => unimplemented!("Error: Unimplemented opcode."),
            0xec => unimplemented!("Error: Unimplemented opcode."),
            0xed => unimplemented!("Error: Unimplemented opcode."),
            0xee => unimplemented!("Error: Unimplemented opcode."),
            0xef => unimplemented!("Error: Unimplemented opcode."),

            0xf0 => unimplemented!("Error: Unimplemented opcode."),
            0xf1 => unimplemented!("Error: Unimplemented opcode."),
            0xf2 => unimplemented!("Error: Unimplemented opcode."),
            0xf3 => unimplemented!("Error: Unimplemented opcode."),
            0xf4 => unimplemented!("Error: Unimplemented opcode."),
            0xf5 => unimplemented!("Error: Unimplemented opcode."),
            0xf6 => unimplemented!("Error: Unimplemented opcode."),
            0xf7 => unimplemented!("Error: Unimplemented opcode."),
            0xf8 => unimplemented!("Error: Unimplemented opcode."),
            0xf9 => unimplemented!("Error: Unimplemented opcode."),
            0xfa => unimplemented!("Error: Unimplemented opcode."),
            0xfb => unimplemented!("Error: Unimplemented opcode."),
            0xfc => unimplemented!("Error: Unimplemented opcode."),
            0xfd => unimplemented!("Error: Unimplemented opcode."),
            0xfe => unimplemented!("Error: Unimplemented opcode."),
            0xff => unimplemented!("Error: Unimplemented opcode."),
        }
    }

    /// Description: The specified register or memory byte is
    /// decremented by one.
    /// Condition bits affected: Zero, Sign, Parity, Ayxiliary
    /// Carry
    fn dcr(&mut self) {
        let instruction = self.memory[self.pc as usize];
        let reg = ((instruction & 0b0011_1000) >> 3) as usize;
        let result = self.registers[reg].wrapping_sub(1);
        self.update_flags(result);
        // self.cc.z = if result == 0 { 1 } else { 0 };
        // self.cc.s = if 0x80 == (result & 0x80) { 1 } else { 0 };
        // self.cc.p = is_parity_even(result);
        self.registers[reg] = result;
        self.pc += 1;
    }

    /// Description: Program execution continues unconditionally
    /// at memory address adr.
    /// Condition bits affected: None
    fn jmp(&mut self) {
        let low_add = self.memory[(self.pc + 1) as usize] as u16;
        let hi_add = self.memory[(self.pc + 2) as usize] as u16;
        let addr = (hi_add << 8) | low_add;
        self.pc = addr;
    }

    /// Description: If the Zero bit is zero, program execution
    /// continues at the memory address adr.
    /// Condition bits affected: None
    fn jnz(&mut self) {
        let low_add = self.memory[(self.pc + 1) as usize] as u16;
        let hi_add = self.memory[(self.pc + 2) as usize] as u16;
        let addr = (hi_add << 8) | low_add;
        if self.cc.z == 0 {
            self.pc = addr;
        } else {
            self.pc += 3;
        }
    }

    /// Description: The third byte of the instruciton is loaded
    /// into the first register of the specified pair, while the
    /// second byte of the instruction is loaded into the second
    /// register of the specified pair. If SP is specified as the
    /// register pair, the second byte of the instruction replaces
    /// the lest significant 8 bits of the stack pointer, while the
    /// third byte of the instruction replaces the most significant
    /// 8 bits of the stack pointer.
    /// Condition bits affected: None.
    fn lxi(&mut self) {
        let instruction = self.memory[self.pc as usize];
        let rp = (instruction & 0b0011_0000) >> 4;
        let low_data = self.memory[(self.pc + 1) as usize];
        let high_data = self.memory[(self.pc + 2) as usize];
        match rp {
            0b00 => {
                self.registers[REG_B] = high_data;
                self.registers[REG_C] = low_data;
            }
            0b01 => {
                self.registers[REG_D] = high_data;
                self.registers[REG_E] = low_data;
            }
            0b10 => {
                self.registers[REG_H] = high_data;
                self.registers[REG_L] = low_data;
            }
            0b11 => {
                self.sp = ((high_data as u16) << 8) | (low_data as u16);
            }
            _ => {
                unreachable!("lxi");
            }
        }
        self.pc += 3;
    }

    /// Description: One byte of data is moved from the
    /// reigster specified by src to the register specified
    /// by dst. The data replaces the contents of the destination
    /// register; the source remains unchanged.
    /// Condition bits affected: None.
    fn mov(&mut self) {
        let isntruction = self.memory[self.pc as usize];
        let dst = (isntruction & 0b0011_1000) >> 3;
        let src = isntruction & 0b0000_0111;
        if dst == M_REF {
            let offset: usize =
                (((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16)) as usize;
            self.memory[offset] = self.registers[src as usize];
        } else if src == M_REF {
            let offset: usize =
                (((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16)) as usize;
            self.registers[dst as usize] = self.memory[offset];
        } else {
            self.registers[dst as usize] = self.registers[src as usize];
        }
        self.pc += 1;
    }

    /// Description: The contents of the memory location
    /// addressed by registers B and C, or by registers D and E,
    /// replace the contents of the accumulator
    /// Condition bits affected: None.
    fn ldax(&mut self) {
        let instruction = self.memory[self.pc as usize];
        let rp = (instruction & 0b0001_0000) >> 4;
        match rp {
            0b0 => {
                let offset: usize = (((self.registers[REG_B] as u16) << 8)
                    | (self.registers[REG_C] as u16)) as usize;
                self.registers[REG_A] = self.memory[offset];
            }
            0b1 => {
                let offset: usize = (((self.registers[REG_D] as u16) << 8)
                    | (self.registers[REG_E] as u16)) as usize;
                self.registers[REG_A] = self.memory[offset];
            }
            _ => {
                unreachable!("ldax");
            }
        }
        self.pc += 1;
    }

    /// Description: The byte of immediate data is stored in
    /// the specified register or memory byte.
    /// Condition bits affected: None.
    fn mvi(&mut self) {
        let instruction = self.memory[self.pc as usize];
        let reg = (instruction & 0b00111000) >> 3;
        let data = self.memory[(self.pc + 1) as usize];
        if reg == M_REF {
            let offset: usize =
                (((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16)) as usize;
            self.memory[offset] = data;
        } else {
            self.registers[reg as usize] = data;
        }
        self.pc += 2;
    }

    /// Description: The 16-bit number held in the specified
    /// register pair is incremented by one.
    /// Condition bits affected: None
    fn inx(&mut self) {
        let instruction = self.memory[self.pc as usize];
        let rp = (instruction & 0b0011_0000) >> 4;
        match rp {
            0b00 => {
                let number = ((self.registers[REG_B] as u16) << 8) | (self.registers[REG_C] as u16);
                let sum = number.wrapping_add(1);
                self.registers[REG_B] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_C] = (sum & 0x00FF) as u8;
            }
            0b01 => {
                let number = ((self.registers[REG_D] as u16) << 8) | (self.registers[REG_E] as u16);
                let sum = number.wrapping_add(1);
                self.registers[REG_D] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_E] = (sum & 0x00FF) as u8;
            }
            0b10 => {
                let number = ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
                let sum = number.wrapping_add(1);
                self.registers[REG_H] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_L] = (sum & 0x00FF) as u8;
            }
            0b11 => {
                self.sp = self.sp.wrapping_add(1);
            }
            _ => {
                unreachable!("inx");
            }
        }
        self.pc += 1;
    }

    /// Description: A call operation is unconditionally performed
    /// to subroutine sub.
    /// Condition bits affected: None
    fn call(&mut self) {
        let low_add: u16 = self.memory[(self.pc + 1) as usize] as u16;
        let hi_add: u16 = self.memory[(self.pc + 2) as usize] as u16;
        let addr: u16 = (hi_add << 8) | low_add;
        let ret_addr: u16 = self.pc + 3;
        let hi_ret_addr: u8 = ((ret_addr & 0xFF00) >> 8) as u8;
        let lo_ret_addr: u8 = (ret_addr & 0x00FF) as u8;
        self.memory[(self.sp - 1) as usize] = lo_ret_addr;
        self.memory[(self.sp - 2) as usize] = hi_ret_addr;
        self.sp -= 2;
        self.pc = addr;
    }
}
