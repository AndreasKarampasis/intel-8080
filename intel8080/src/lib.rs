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
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    a: u8,
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
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            a: 0,
            registers: [0; REGISTER_NUM],
            memory: [0; MEMORY_SIZE],
            pc: 0,
            sp: 0,
            cc: ConditionCodes::new(),
            interrupts_enable: false,
        }
    }

    pub fn test(&mut self) {
        self.print_state();
        self.memory[0] = 0xc6;
        self.registers[REG_A] = 1;
        self.memory[1] = 0xFF;
        self.op_adi();
        self.print_state();
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
            "A        |{}\t|{:#06X}\t|{:#010b}        |",
            self.registers[REG_A], self.registers[REG_A], self.registers[REG_A]
        );
        println!(
            "B        |{}\t|{:#06X}\t|{:#010b}        |",
            self.registers[REG_B], self.registers[REG_B], self.registers[REG_B]
        );
        println!(
            "C        |{}\t|{:#06X}\t|{:#010b}        |",
            self.registers[REG_C], self.registers[REG_C], self.registers[REG_C]
        );
        println!(
            "D        |{}\t|{:#06X}\t|{:#010b}        |",
            self.registers[REG_D], self.registers[REG_D], self.registers[REG_D]
        );
        println!(
            "E        |{}\t|{:#06X}\t|{:#010b}        |",
            self.registers[REG_E], self.registers[REG_E], self.registers[REG_E]
        );
        println!(
            "H        |{}\t|{:#06X}\t|{:#010b}        |",
            self.registers[REG_H], self.registers[REG_H], self.registers[REG_H]
        );
        println!(
            "L        |{}\t|{:#06X}\t|{:#010b}        |",
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
            0x00 => {}
            0x01 => self.op_lxi_b(),
            0x02 => unimplemented!("Error: Unimplemented opcode."),
            0x03 => unimplemented!("Error: Unimplemented opcode."),
            0x04 => self.op_inr(),
            0x05 => self.op_dcr(),
            0x06 => self.op_mvi_b(),
            0x07 => self.op_rlc(),
            0x08 => unimplemented!("Error: Unimplemented opcode."),
            0x09 => self.op_dad(),
            0x0a => unimplemented!("Error: Unimplemented opcode."),
            0x0b => unimplemented!("Error: Unimplemented opcode."),
            0x0c => unimplemented!("Error: Unimplemented opcode."),
            0x0d => self.op_dcr(),
            0x0e => self.op_mvi_c(),
            0x0f => self.op_rrc(),

            0x10 => unimplemented!("Error: Unimplemented opcode."),
            0x11 => self.op_lxi_d(),
            0x12 => unimplemented!("Error: Unimplemented opcode."),
            0x13 => self.op_inx_d(),
            0x14 => unimplemented!("Error: Unimplemented opcode."),
            0x15 => unimplemented!("Error: Unimplemented opcode."),
            0x16 => unimplemented!("Error: Unimplemented opcode."),
            0x17 => unimplemented!("Error: Unimplemented opcode."),
            0x18 => unimplemented!("Error: Unimplemented opcode."),
            0x19 => self.op_dad(),
            0x1a => self.op_ldax_d(),
            0x1b => unimplemented!("Error: Unimplemented opcode."),
            0x1c => unimplemented!("Error: Unimplemented opcode."),
            0x1d => unimplemented!("Error: Unimplemented opcode."),
            0x1e => unimplemented!("Error: Unimplemented opcode."),
            0x1f => unimplemented!("Error: Unimplemented opcode."),

            0x20 => unimplemented!("Error: Unimplemented opcode."),
            0x21 => self.op_lxi_h(),
            0x22 => unimplemented!("Error: Unimplemented opcode."),
            0x23 => self.op_inx_h(),
            0x24 => unimplemented!("Error: Unimplemented opcode."),
            0x25 => unimplemented!("Error: Unimplemented opcode."),
            0x26 => self.op_mvi_h(),
            0x27 => unimplemented!("Error: Unimplemented opcode."),
            0x28 => unimplemented!("Error: Unimplemented opcode."),
            0x29 => self.op_dad(),
            0x2a => unimplemented!("Error: Unimplemented opcode."),
            0x2b => unimplemented!("Error: Unimplemented opcode."),
            0x2c => unimplemented!("Error: Unimplemented opcode."),
            0x2d => unimplemented!("Error: Unimplemented opcode."),
            0x2e => unimplemented!("Error: Unimplemented opcode."),
            0x2f => self.op_cma(),

            0x30 => unimplemented!("Error: Unimplemented opcode."),
            0x31 => self.op_lxi_sp(),
            0x32 => self.op_sta(),
            0x33 => unimplemented!("Error: Unimplemented opcode."),
            0x34 => unimplemented!("Error: Unimplemented opcode."),
            0x35 => unimplemented!("Error: Unimplemented opcode."),
            0x36 => self.op_mvi_m(),
            0x37 => self.op_stc(),
            0x38 => unimplemented!("Error: Unimplemented opcode."),
            0x39 => self.op_dad(),
            0x3a => self.op_lda(),
            0x3b => unimplemented!("Error: Unimplemented opcode."),
            0x3c => unimplemented!("Error: Unimplemented opcode."),
            0x3d => unimplemented!("Error: Unimplemented opcode."),
            0x3e => self.op_mvi_a(),
            0x3f => self.op_cmc(),

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
            0x56 => self.op_mov(),
            0x57 => unimplemented!("Error: Unimplemented opcode."),
            0x58 => unimplemented!("Error: Unimplemented opcode."),
            0x59 => unimplemented!("Error: Unimplemented opcode."),
            0x5a => unimplemented!("Error: Unimplemented opcode."),
            0x5b => unimplemented!("Error: Unimplemented opcode."),
            0x5c => unimplemented!("Error: Unimplemented opcode."),
            0x5d => unimplemented!("Error: Unimplemented opcode."),
            0x5e => self.op_mov(),
            0x5f => unimplemented!("Error: Unimplemented opcode."),

            0x60 => unimplemented!("Error: Unimplemented opcode."),
            0x61 => unimplemented!("Error: Unimplemented opcode."),
            0x62 => unimplemented!("Error: Unimplemented opcode."),
            0x63 => unimplemented!("Error: Unimplemented opcode."),
            0x64 => unimplemented!("Error: Unimplemented opcode."),
            0x65 => unimplemented!("Error: Unimplemented opcode."),
            0x66 => self.op_mov(),
            0x67 => unimplemented!("Error: Unimplemented opcode."),
            0x68 => unimplemented!("Error: Unimplemented opcode."),
            0x69 => unimplemented!("Error: Unimplemented opcode."),
            0x6a => unimplemented!("Error: Unimplemented opcode."),
            0x6b => unimplemented!("Error: Unimplemented opcode."),
            0x6c => unimplemented!("Error: Unimplemented opcode."),
            0x6d => unimplemented!("Error: Unimplemented opcode."),
            0x6e => unimplemented!("Error: Unimplemented opcode."),
            0x6f => self.op_mov(),

            0x70 => unimplemented!("Error: Unimplemented opcode."),
            0x71 => unimplemented!("Error: Unimplemented opcode."),
            0x72 => unimplemented!("Error: Unimplemented opcode."),
            0x73 => unimplemented!("Error: Unimplemented opcode."),
            0x74 => unimplemented!("Error: Unimplemented opcode."),
            0x75 => unimplemented!("Error: Unimplemented opcode."),
            0x76 => unimplemented!("Error: Unimplemented opcode."),
            0x77 => self.op_mov(),
            0x78 => unimplemented!("Error: Unimplemented opcode."),
            0x79 => unimplemented!("Error: Unimplemented opcode."),
            0x7a => self.op_mov(),
            0x7b => self.op_mov(),
            0x7c => self.op_mov(),
            0x7d => unimplemented!("Error: Unimplemented opcode."),
            0x7e => self.op_mov(),
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
            0xa7 => self.op_ana(),
            0xa8 => unimplemented!("Error: Unimplemented opcode."),
            0xa9 => unimplemented!("Error: Unimplemented opcode."),
            0xaa => unimplemented!("Error: Unimplemented opcode."),
            0xab => unimplemented!("Error: Unimplemented opcode."),
            0xac => unimplemented!("Error: Unimplemented opcode."),
            0xad => unimplemented!("Error: Unimplemented opcode."),
            0xae => unimplemented!("Error: Unimplemented opcode."),
            0xaf => self.op_xra(),

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
            0xc1 => self.op_pop(),
            0xc2 => self.op_jnz(),
            0xc3 => self.op_jmp(),
            0xc4 => unimplemented!("Error: Unimplemented opcode."),
            0xc5 => self.op_push(),
            0xc6 => self.op_adi(),
            0xc7 => unimplemented!("Error: Unimplemented opcode."),
            0xc8 => unimplemented!("Error: Unimplemented opcode."),
            0xc9 => self.op_ret(),
            0xca => unimplemented!("Error: Unimplemented opcode."),
            0xcb => unimplemented!("Error: Unimplemented opcode."),
            0xcc => unimplemented!("Error: Unimplemented opcode."),
            0xcd => self.op_call(),
            0xce => unimplemented!("Error: Unimplemented opcode."),
            0xcf => unimplemented!("Error: Unimplemented opcode."),

            0xd0 => unimplemented!("Error: Unimplemented opcode."),
            0xd1 => self.op_pop(),
            0xd2 => unimplemented!("Error: Unimplemented opcode."),
            0xd3 => self.op_out(),
            0xd4 => unimplemented!("Error: Unimplemented opcode."),
            0xd5 => self.op_push(),
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
            0xe1 => self.op_pop(),
            0xe2 => unimplemented!("Error: Unimplemented opcode."),
            0xe3 => unimplemented!("Error: Unimplemented opcode."),
            0xe4 => unimplemented!("Error: Unimplemented opcode."),
            0xe5 => self.op_push(),
            0xe6 => self.op_ani(),
            0xe7 => unimplemented!("Error: Unimplemented opcode."),
            0xe8 => unimplemented!("Error: Unimplemented opcode."),
            0xe9 => unimplemented!("Error: Unimplemented opcode."),
            0xea => unimplemented!("Error: Unimplemented opcode."),
            0xeb => {
                println!("XCHG");
            }
            0xec => unimplemented!("Error: Unimplemented opcode."),
            0xed => unimplemented!("Error: Unimplemented opcode."),
            0xee => unimplemented!("Error: Unimplemented opcode."),
            0xef => unimplemented!("Error: Unimplemented opcode."),

            0xf0 => unimplemented!("Error: Unimplemented opcode."),
            0xf1 => self.op_pop(),
            0xf2 => unimplemented!("Error: Unimplemented opcode."),
            0xf3 => self.op_di(),
            0xf4 => unimplemented!("Error: Unimplemented opcode."),
            0xf5 => self.op_push(),
            0xf6 => unimplemented!("Error: Unimplemented opcode."),
            0xf7 => unimplemented!("Error: Unimplemented opcode."),
            0xf8 => unimplemented!("Error: Unimplemented opcode."),
            0xf9 => unimplemented!("Error: Unimplemented opcode."),
            0xfa => unimplemented!("Error: Unimplemented opcode."),
            0xfb => self.op_ei(),
            0xfc => unimplemented!("Error: Unimplemented opcode."),
            0xfd => unimplemented!("Error: Unimplemented opcode."),
            0xfe => self.op_cpi(),
            0xff => unimplemented!("Error: Unimplemented opcode."),
        }
        self.pc += 1;
    }

    // TODO: Move all the operation implementations to a different file.

    fn op_mvi_b(&mut self) {
        // B <- byte2
        let byte2 = self.memory[(self.pc as usize) + 1];
        self.b = byte2;
        self.pc += 2;
    }

    fn op_mvi_c(&mut self) {
        let byte2 = self.memory[(self.pc + 1) as usize];
        self.c = byte2;
        self.pc += 2;
    }

    fn op_mvi_h(&mut self) {
        let byte2 = self.memory[(self.pc + 1) as usize];
        self.h = byte2;
        self.pc += 2;
    }

    /// The byte of immediate data is stored in the specified memory byte
    /// Condition bits affected: None
    fn op_mvi_m(&mut self) {
        let byte2: u8 = self.memory[(self.pc + 1) as usize];
        let offset: u16 = ((self.h as u16) << 8) | (self.l as u16);
        self.memory[offset as usize] = byte2;
        self.pc += 2;
    }

    /// The byte of immediate data is stored in the accumulator
    /// Condition bits affected: None
    fn op_mvi_a(&mut self) {
        let byte2 = self.memory[(self.pc + 1) as usize];
        self.a = byte2;
        self.pc += 2;
    }
    fn op_lxi_b(&mut self) {
        // B <- byte3, C <- byte2
        let byte2 = self.memory[(self.pc as usize) + 1];
        let byte3 = self.memory[(self.pc as usize) + 2];
        self.b = byte3;
        self.c = byte2;
        self.pc += 3;
    }
    /// This instruction loads the register pair DE with a 16-bit
    /// address formed by the immediate 8-bit values in the next
    /// two memory locations.
    fn op_lxi_d(&mut self) {
        let byte2 = self.memory[(self.pc + 1) as usize];
        let byte3 = self.memory[(self.pc + 2) as usize];
        self.d = byte3;
        self.e = byte2;
        self.pc += 3;
    }

    fn op_inx_d(&mut self) {
        let de_reg_pair = ((self.d as u16) << 8) | (self.e as u16);
        let sum = de_reg_pair + 1;
        self.d = ((sum & 0xFF00) >> 8) as u8;
        self.e = (sum & 0x00FF) as u8;
    }

    /// Load accumulator with the contents of the memory location DE
    fn op_ldax_d(&mut self) {
        let de = ((self.d as u16) << 8) | (self.e as u16);
        // What happens if offset is out of memory bounds?
        self.a = self.memory[de as usize];
    }

    fn op_lxi_h(&mut self) {
        // H <- byte 3, L <- byte 2
        let byte2 = self.memory[(self.pc + 1) as usize];
        let byte3 = self.memory[(self.pc + 2) as usize];
        self.h = byte3;
        self.l = byte2;
        self.pc += 3;
    }

    /// Increments the HL register pair.
    fn op_inx_h(&mut self) {
        let hl_reg_pair = ((self.h as u16) << 8) | (self.l as u16);
        let sum = hl_reg_pair + 1;
        self.h = ((sum & 0xFF00) >> 8) as u8;
        self.l = (sum & 0x00FF) as u8;
    }

    fn op_lxi_sp(&mut self) {
        let byte2 = self.memory[(self.pc + 1) as usize];
        let byte3 = self.memory[(self.pc + 2) as usize];
        self.sp = ((byte3 as u16) << 8) | (byte2 as u16);
        self.pc += 3;
    }

    /// Description: Rotate Accumulator Left: The carry bit is set equal
    /// to the MS of the accumulator. The contents of the accumulator are
    /// rotated one bit position to the left, with the MSB being transffered
    /// to the LSB position of the accumulator.
    fn op_rlc(&mut self) {
        self.cc.cy = (self.registers[REG_A] & 0b1000_0000) >> 7;
        self.registers[REG_A] = (self.registers[REG_A] << 1) | self.cc.cy;
    }

    /// Description: Rotate Accumulator Right: The carry bit is set equal to the LS
    /// of the accumulator. The contents of the accumulator are rotated
    /// one bit position to the right, with the LSB being transffered
    /// to the MSB position of the accumulator.
    fn op_rrc(&mut self) {
        self.cc.cy = self.registers[REG_A] & 0b0000_0001;
        self.registers[REG_A] = (self.registers[REG_A] >> 1) | (self.cc.cy << 7);
    }

    /// Description: The contents of the accumulaltor replace
    /// the byte at the memory address formed by concatenating
    /// HI ADD with LOW ADD
    /// Condition bits affected: None
    fn op_sta(&mut self) {
        let lo = self.memory[(self.pc + 1) as usize];
        let hi = self.memory[(self.pc + 2) as usize];

        let offset = ((hi as u16) << 8) | (lo as u16);
        self.memory[offset as usize] = self.a;
        self.pc += 3;
    }

    /// Description: The byte at the memory address formed
    /// by concatenating HI ADD with LOW ADD replaces the
    /// contents of the accumulator.
    /// Condition bits affected: None
    fn op_lda(&mut self) {
        let lo = self.memory[(self.pc + 1) as usize];
        let hi = self.memory[(self.pc + 2) as usize];

        let offset = ((hi as u16) << 8) | (lo as u16);
        self.a = self.memory[offset as usize];

        self.pc += 3;
    }

    /// Description: One byte of data is moved from the register specified by src to
    /// the dst register. The data replaces the contents of the destination
    /// register; the source remains unchanged.
    /// Condition bits affected: None
    fn op_mov(&mut self) {
        let byte = self.memory[self.pc as usize];
        let src_reg = byte & 0b0000_0111;
        let dst_reg = (byte & 0b0011_1000) >> 3;

        if dst_reg == M_REF {
            let offset: u16 =
                ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
            self.memory[offset as usize] = self.registers[src_reg as usize];
        } else if src_reg == M_REF {
            let offset: u16 =
                ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
            self.registers[src_reg as usize] = self.memory[offset as usize];
        } else {
            self.registers[dst_reg as usize] = self.registers[src_reg as usize];
        }
    }

    /// Description: If the Carry bit is 0, it is set to 1. If the Carry bit = 1,
    /// it is reset to 0.
    /// Condition bits affected: Carry
    fn op_cmc(&mut self) {
        self.cc.cy ^= 1;
    }

    /// Description: The Carry bit is set to one
    /// Condition bits affected: Carry
    fn op_stc(&mut self) {
        self.cc.cy = 0x01;
    }

    /// Description: The specified register or memory byte is incremented by one.
    /// Condition bits affected: Zero, SIgn, Parity, Auxiliary Carry
    fn op_inr(&mut self) {
        let reg = (self.memory[self.pc as usize] & 0b0011_1000) >> 3;
        if reg == M_REF {
            let offset: u16 =
                ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
            let (sum, carry) = self.memory[offset as usize].overflowing_add(1);
            self.memory[offset as usize] = sum;
            self.update_flags(self.memory[offset as usize]);
            self.cc.cy = if carry { 1 } else { 0 };
        } else {
            let (sum, carry) = self.registers[reg as usize].overflowing_add(1);
            self.memory[reg as usize] = sum;
            self.update_flags(self.registers[reg as usize]);
            self.cc.cy = if carry { 1 } else { 0 };
        }
    }

    /// Description: The specified register or memory byte is decremented by one.
    /// Condition bits affected: Zero, Sign, Parity, Auxiliary Carry
    fn op_dcr(&mut self) {
        // B <- B - 1
        let op = self.memory[self.pc as usize];
        let reg = (op & 0b0011_1000) >> 3;

        if reg == M_REF {
            let offset: u16 =
                ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
            self.memory[offset as usize] = self.memory[offset as usize].wrapping_sub(1);
            self.update_flags(self.memory[offset as usize]);
            self.cc.cy = 0;
        } else {
            self.registers[reg as usize] = self.registers[reg as usize].wrapping_sub(1);
            self.update_flags(self.registers[reg as usize]);
            self.cc.cy = 0;
        }
    }

    /// Description: Each bit of the contents of the accumulator is
    /// complemented.
    /// Condition bits affected: None
    fn op_cma(&mut self) {
        self.registers[REG_A] = !self.registers[REG_A];
    }

    /// The 16-bit number in the specified register pair is added
    /// to the 16-bit number held in the H and L registers using
    /// two's complement arithmetic. The result replaces the contents
    /// of the H and L registers.
    /// Condition bits affected: Carry (cy)
    fn op_dad(&mut self) {
        let op: u8 = self.memory[self.pc as usize];
        let reg_pair: u8 = (op & 0b0011_0000) >> 4;
        let hl: u16 = ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);

        match reg_pair {
            0b00 => {
                let bc: u16 =
                    ((self.registers[REG_B] as u16) << 8) | (self.registers[REG_C] as u16);
                let (sum, carry) = hl.overflowing_add(bc);
                self.registers[REG_H] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_L] = (sum & 0x00FF) as u8;
                self.cc.cy = if carry { 1 } else { 0 };
            }
            0b01 => {
                let de: u16 =
                    ((self.registers[REG_D] as u16) << 8) | (self.registers[REG_E] as u16);
                let (sum, carry) = hl.overflowing_add(de);
                self.registers[REG_H] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_L] = (sum & 0x00FF) as u8;
                self.cc.cy = if carry { 1 } else { 0 };
            }
            0b10 => {
                let (sum, carry) = hl.overflowing_add(hl);
                self.registers[REG_H] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_L] = (sum & 0x00FF) as u8;
                self.cc.cy = if carry { 1 } else { 0 };
            }
            0b11 => {
                let (sum, carry) = hl.overflowing_add(self.sp);
                self.registers[REG_H] = ((sum & 0xFF00) >> 8) as u8;
                self.registers[REG_L] = (sum & 0x00FF) as u8;
                self.cc.cy = if carry { 1 } else { 0 };
            }
            _ => {
                unreachable!();
            }
        }
    }

    /// Description: The specified byte is logically ANDed bit by bit
    /// with the contents of the accumulator. The carry bit is reset to zero
    /// Condition bits affected: Carry, Zero, Sign, Parity
    fn op_ana(&mut self) {
        let op = self.memory[self.pc as usize];
        let reg = op & 0b0000_0111;

        if reg == M_REF {
            let offset: u16 =
                ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
            self.registers[REG_A] &= self.memory[offset as usize];
        } else {
            self.registers[REG_A] &= self.registers[reg as usize];
        }
        self.update_flags(self.registers[REG_A]);
        self.cc.cy = 0;
    }

    /// Description: The specified byte is EXCLUSIVE-ORed but by bit
    /// with the contents of the accumulator. The Carry bit is reset to zero
    /// Condition bits affected: Carry, Zero, Sign, Parity, Auxiliary Carry
    fn op_xra(&mut self) {
        let op = self.memory[self.pc as usize];
        let reg = op & 0b0000_0111;

        match reg {
            M_REF => {
                let offset: u16 =
                    ((self.registers[REG_H] as u16) << 8) | (self.registers[REG_L] as u16);
                self.registers[REG_A] ^= self.memory[offset as usize];
            }
            _ => {
                self.registers[REG_A] ^= self.registers[reg as usize];
            }
        }
        self.update_flags(self.registers[REG_A]);
        // TODO: update ac flag in the future? probably not gonna happen
        self.cc.cy = 0;
    }

    /// Description: The contents of the specified register pair
    /// are saved in two bytes of memory indicated by the stack
    /// pointer SP. The contents of the first register are saved
    /// at the memory address one less than the address indicated
    /// by the stack pointer; the contents of the second register
    /// are saved at the address two less than the address indicated
    /// by the stack pointer. If register PSW is specified, the first
    /// byte of information saved holds the contents of the A register;
    /// the second byte holds the settings of the five condition bits,
    /// Condition bits affected: None
    fn op_push(&mut self) {
        let op = self.memory[self.pc as usize];
        let reg_pair = (op & 0b00110000) >> 4;
        match reg_pair {
            0b00 => {
                self.memory[(self.sp + 1) as usize] = self.registers[REG_B];
                self.memory[self.sp as usize] = self.registers[REG_C];
            }
            0b01 => {
                self.memory[(self.sp + 1) as usize] = self.registers[REG_D];
                self.memory[self.sp as usize] = self.registers[REG_E];
            }
            0b10 => {
                self.memory[(self.sp + 1) as usize] = self.registers[REG_H];
                self.memory[self.sp as usize] = self.registers[REG_L];
            }
            0b11 => {
                self.memory[(self.sp + 1) as usize] = self.registers[REG_A];
                let mut flags: u8 = 0b0000_0000;
                flags = flags | (self.cc.s << 7);
                flags = flags | (self.cc.z << 6);
                flags = flags | (self.cc.ac << 4);
                flags = flags | (self.cc.p << 2);
                flags = flags | 0b0000_0010;
                flags = flags | self.cc.cy;
                self.memory[self.sp as usize] = flags;
            }
            _ => {
                unreachable!("Aborted");
            }
        }
        self.sp -= 2;
    }

    /// Description: The contents of the specified register pair are
    /// restored from two bytes of memory indicated by the stack pointer SP.
    /// The byte of data at the memory address indicated by the stack pointer
    /// is loaded into the second register of the register pair; the byte of
    /// data at the address one greater than the address indicated by the stack
    /// pointer is loaded into the first register pair. If register pair PSW is
    /// specified, the byte of data indicated by the contents of the stack
    /// pointer plus one is used to restore the values of the five condition bits
    /// Condition bits affected: If register pair PSW is specified, Carry, Sign
    /// Zero, Parity and Auxiliary Carry may be changed. Other wise, none affected
    fn op_pop(&mut self) {
        let op = self.memory[self.pc as usize];
        let reg_pair = (op & 0b0011_0000) >> 4;
        match reg_pair {
            0b00 => {
                self.registers[REG_B] = self.memory[(self.sp + 1) as usize];
                self.registers[REG_C] = self.memory[self.sp as usize];
            }
            0b01 => {
                self.registers[REG_D] = self.memory[(self.sp + 1) as usize];
                self.registers[REG_E] = self.memory[self.sp as usize];
            }
            0b10 => {
                self.registers[REG_H] = self.memory[(self.sp + 1) as usize];
                self.registers[REG_L] = self.memory[self.sp as usize];
            }
            0b11 => {
                self.registers[REG_A] = self.memory[(self.sp + 1) as usize];
                let psw = self.memory[self.sp as usize];
                self.cc.s = (psw & 0b1000_0000) >> 7;
                self.cc.z = (psw & 0b0100_0000) >> 6;
                self.cc.ac = (psw & 0b0001_0000) >> 4;
                self.cc.p = (psw & 0b0000_0100) >> 2;
                self.cc.cy = psw & 0b0000_0001;
            }
            _ => {
                unreachable!("Aborted");
            }
        }
        self.sp += 2;
    }

    /// Description: Program execution continues unconditionally
    /// at memory address adr.
    /// Condition bits affected: None
    fn op_jmp(&mut self) {
        let low_addr: u16 = self.memory[(self.pc + 1) as usize] as u16;
        let high_addr: u16 = self.memory[(self.pc + 2) as usize] as u16;
        let addr: u16 = (high_addr << 8) | low_addr;
        self.pc = addr;
    }

    /// Description:If the Zero bit is zero, program execution
    /// continues at the memory addr.
    /// Condition bits affected: None
    fn op_jnz(&mut self) {
        let low_addr: u16 = self.memory[(self.pc + 1) as usize] as u16;
        let high_addr: u16 = self.memory[(self.pc + 2) as usize] as u16;
        let addr: u16 = (high_addr << 8) | low_addr;
        if self.cc.z == 0 {
            self.pc = addr;
        }
    }

    /// Description: The byte of immediate data is added to
    /// the contents of the accumulator using two's complement
    /// arithmetic.
    /// Condition bits affected: Carry, Sign, Zero, Parity,
    /// Auxiliary Carry
    fn op_adi(&mut self) {
        let imm_data = self.memory[(self.pc + 1) as usize];
        let (sum, carry) = self.registers[REG_A].overflowing_add(imm_data);
        self.registers[REG_A] = sum;
        self.cc.cy = if carry { 1 } else { 0 };
        self.update_flags(self.registers[REG_A]);
        self.pc += 1;
    }
    /// Description: The byte of immediate data is logically
    /// to ANDed with the contents of the accumulator. The Carry bit
    /// is reset to zero.
    /// Condition bits affected: Carry, Sign, Zero, Parity,
    /// Auxiliary Carry
    fn op_ani(&mut self) {
        let imm_data = self.memory[(self.pc + 1) as usize];
        self.registers[REG_A] = self.registers[REG_A] & imm_data;
        self.cc.cy = 0;
        self.update_flags(self.registers[REG_A]);
        self.pc += 1;
    }

    /// Description: The byte of immediate data is compared
    /// to the contents of the accumulator.
    /// Condition bits affected: Carry, Zero, Sign, Parity,
    /// Auxiliary Carry
    fn op_cpi(&mut self) {
        let imm_data = self.memory[(self.pc + 1) as usize];
        let (res, carry) = self.registers[REG_A].overflowing_sub(imm_data);
        self.update_flags(res);
        self.cc.cy = if carry { 1 } else { 0 };
        self.pc += 1;
    }

    /// Description: A call operation is unconditionally
    /// performed to subroutine sub.
    /// Condition bits affected: None
    fn op_call(&mut self) {
        let ret: u16 = self.pc + 2;
        self.memory[(self.sp - 1) as usize] = ((ret & 0xff00) >> 8) as u8;
        self.memory[(self.sp - 2) as usize] = (ret & 0x00ff) as u8;

        let low_addr: u16 = self.memory[(self.pc + 1) as usize] as u16;
        let hi_addr: u16 = self.memory[(self.pc + 2) as usize] as u16;
        self.sp -= 2;
        self.pc = (hi_addr << 8) | low_addr;
    }

    /// Description: A return operation is unconditionally
    /// performed. Thus, execution proceeds with the instruction
    /// immediately following the last call instruction.
    /// Condition bits affected: None
    fn op_ret(&mut self) {
        let ret_lo_add = self.memory[(self.sp + 1) as usize] as u16;
        let ret_hi_add = self.memory[(self.sp) as usize] as u16;
        let ret_add = (ret_hi_add << 8) | ret_lo_add;
        self.pc = ret_add;
    }

    /// Description: The contents of the accumulator are sent
    /// to output device number exp.
    /// Condition bits affected: None
    fn op_out(&mut self) {
        let exp = self.memory[(self.pc + 1) as usize];
        // TODO:
        // IN and OUT are instructions that the 8080 hardware
        // used to talk to external hardware. For now, implement
        // these but make them do nothing besides skip over its
        //data byte.
        // output[exp] = self.registers[REG_A];
        self.pc += 1;
    }

    /// Description: This instruction sets the INTE flip-flop
    /// enabling the CPU to recognise and respond to interrupts.
    /// Condition bits afftected: None
    fn op_ei(&mut self) {
        self.interrupts_enable = true;
    }

    /// Description: This instruction resets the INTE flip-flop
    /// causing the CPU to ignore all interrupts.
    /// Condition bits afftected: None
    fn op_di(&mut self) {
        self.interrupts_enable = true;
    }
}
