const MEMORY_SIZE: usize = 65_536;

pub struct Intel8080 {
    //regs: [u8; NUM_REGS],
    memory: [u8; MEMORY_SIZE],
    pc: u16,
    //    sp: u16,
    //cc: ConditionCodes,
    // int_enable: bool,
}

impl Intel8080 {
    pub fn new() -> Self {
        Self {
            //            regs: [0; NUM_REGS],
            memory: [0; MEMORY_SIZE],
            pc: 0,
            //           sp: 0,
            //            cc: ConditionCodes::new(),
            //            int_enable: false,
        }
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
        self.pc += self.execute(op);
    }

    // TODO: use array of function pointers for better visibility
    pub fn execute(&mut self, op: u8) -> u16 {
        print!("{:#06X} {:02X} ", self.pc, op);
        let mut op_bytes = 1;
        match op {
            0x00 => {
                println!("NOP");
            }
            0x01 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("LXI B, ${:#02X}{:#02X}", byte3, byte2);
            }
            0x02 => {
                println!("STAX B");
            }
            0x03 => {
                println!("INX B");
            }
            0x04 => {
                println!("INR B");
            }
            0x05 => {
                println!("DCR B");
            }
            0x06 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI B, #${:02X}", byte2);
            }
            0x07 => {
                println!("RLC B");
            }
            0x08 => {
                println!("NOP");
            }
            0x09 => {
                println!("DAD B");
            }
            0x0a => {
                println!("LDAX B");
            }
            0x0b => {
                println!("DCX B");
            }
            0x0c => {
                println!("INR C");
            }
            0x0d => {
                println!("DCR C");
            }
            0x0e => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI C, #${:02X}", byte2);
            }
            0x0f => {
                println!("RRC");
            }
            0x10 => {
                println!("NOP");
            }
            0x11 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("LXI D, #${:02X}{:02X}", byte3, byte2);
            }
            0x12 => {
                println!("STAX D");
            }
            0x13 => {
                println!("INX D");
            }
            0x14 => {
                println!("INR D");
            }
            0x15 => {
                println!("DCR D");
            }
            0x16 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI D, #${:02X}", byte2);
            }
            0x17 => {
                println!("RAL");
            }
            0x18 => {
                println!("NOP");
            }
            0x19 => {
                println!("DAD D");
            }
            0x1a => {
                println!("LDAX D");
            }
            0x1b => {
                println!("DCX D");
            }
            0x1c => {
                println!("INR E");
            }
            0x1d => {
                println!("DCR E");
            }
            0x1e => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI E, #${:02X}", byte2);
            }
            0x1f => {
                println!("RAR");
            }
            0x20 => {
                println!("NOP");
            }
            0x21 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("LXI H, #${:02X}{:02X}", byte3, byte2);
            }
            0x22 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("SHLD #{:02X}{:02X}", byte3, byte2);
            }
            0x23 => {
                println!("INX H");
            }
            0x24 => {
                println!("INR H");
            }
            0x25 => {
                println!("DCR H");
            }
            0x26 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI H, #${:02X}", byte2);
            }
            0x27 => {
                println!("DAA");
            }
            0x28 => {
                println!("NOP");
            }
            0x29 => {
                println!("DAD H");
            }
            0x2a => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("LHLD #{:02X}{:02X}", byte3, byte2);
            }
            0x2b => {
                println!("DCX H");
            }
            0x2c => {
                println!("INR L");
            }
            0x2d => {
                println!("DCR L");
            }
            0x2e => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI L, #${:02X}", byte2);
            }
            0x2f => {
                println!("CMA");
            }
            0x30 => {
                println!("NOP");
            }
            0x31 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("LXI SP,  #{:02X}{:02X}", byte3, byte2);
            }
            0x32 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("STA  #{:02X}{:02X}", byte3, byte2);
            }
            0x33 => {
                println!("INX SP");
            }
            0x34 => {
                println!("INX M");
            }
            0x35 => {
                println!("DCR M");
            }
            0x36 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI M, #${:02X}", byte2);
            }
            0x37 => {
                println!("STC");
            }
            0x38 => {
                println!("NOP");
            }
            0x39 => {
                println!("DAD SP");
            }
            0x3a => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("LDA ${:02X}{:02X}", byte3, byte2);
            }
            0x3b => {
                println!("DCX SP");
            }
            0x3c => {
                println!("INR A");
            }
            0x3d => {
                println!("DCR A");
            }
            0x3e => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("MVI A, #${:02X}", byte2);
            }
            0x3f => {
                println!("CMC");
            }
            0x40 => {
                println!("MOV B, B");
            }
            0x41 => {
                println!("MOV B, C");
            }
            0x42 => {
                println!("MOV B, D");
            }
            0x43 => {
                println!("MOV B, E");
            }
            0x44 => {
                println!("MOV B, H");
            }
            0x45 => {
                println!("MOV B, L");
            }
            0x46 => {
                println!("MOV B, M");
            }
            0x47 => {
                println!("MOV B, A");
            }
            0x48 => {
                println!("MOV C, B");
            }
            0x49 => {
                println!("MOV C, C");
            }
            0x4a => {
                println!("MOV C, D");
            }
            0x4b => {
                println!("MOV C, E");
            }
            0x4c => {
                println!("MOV C, H");
            }
            0x4d => {
                println!("MOV C, L");
            }
            0x4e => {
                println!("MOV C, M");
            }
            0x4f => {
                println!("MOV C, A");
            }
            0x50 => {
                println!("MOV D, B");
            }
            0x51 => {
                println!("MOV D, C");
            }
            0x52 => {
                println!("MOV D, D");
            }
            0x53 => {
                println!("MOV D, E");
            }
            0x54 => {
                println!("MOV D, H");
            }
            0x55 => {
                println!("MOV D, L");
            }
            0x56 => {
                println!("MOV D, M");
            }
            0x57 => {
                println!("MOV D, A");
            }
            0x58 => {
                println!("MOV E, B");
            }
            0x59 => {
                println!("MOV E, C");
            }
            0x5a => {
                println!("MOV E, D");
            }
            0x5b => {
                println!("MOV E, E");
            }
            0x5c => {
                println!("MOV E, H");
            }
            0x5d => {
                println!("MOV E, L");
            }
            0x5e => {
                println!("MOV E, M");
            }
            0x5f => {
                println!("MOV E, A");
            }
            0x60 => {
                println!("MOV H, B");
            }
            0x61 => {
                println!("MOV H, C");
            }
            0x62 => {
                println!("MOV H, D");
            }
            0x63 => {
                println!("MOV H, E");
            }
            0x64 => {
                println!("MOV H, H");
            }
            0x65 => {
                println!("MOV H, L");
            }
            0x66 => {
                println!("MOV H, M");
            }
            0x67 => {
                println!("MOV H, A");
            }
            0x68 => {
                println!("MOV L, B");
            }
            0x69 => {
                println!("MOV L, C");
            }
            0x6a => {
                println!("MOV L, D");
            }
            0x6b => {
                println!("MOV L, E");
            }
            0x6c => {
                println!("MOV L, H");
            }
            0x6d => {
                println!("MOV L, L");
            }
            0x6e => {
                println!("MOV L, M");
            }
            0x6f => {
                println!("MOV L, A");
            }
            0x70 => {
                println!("MOV M, B");
            }
            0x71 => {
                println!("MOV M, C");
            }
            0x72 => {
                println!("MOV M, D");
            }
            0x73 => {
                println!("MOV M, E");
            }
            0x74 => {
                println!("MOV M, H");
            }
            0x75 => {
                println!("MOV M, L");
            }
            0x76 => {
                println!("HLT");
            }
            0x77 => {
                println!("MOV M, A");
            }
            0x78 => {
                println!("MOV A, B");
            }
            0x79 => {
                println!("MOV A, C");
            }
            0x7a => {
                println!("MOV A, D");
            }
            0x7b => {
                println!("MOV A, E");
            }
            0x7c => {
                println!("MOV A, H");
            }
            0x7d => {
                println!("MOV A, L");
            }
            0x7e => {
                println!("MOV A, M");
            }
            0x7f => {
                println!("MOV A, A");
            }
            0x80 => {
                println!("ADD B");
            }
            0x81 => {
                println!("ADD C");
            }
            0x82 => {
                println!("ADD D");
            }
            0x83 => {
                println!("ADD E");
            }
            0x84 => {
                println!("ADD H");
            }
            0x85 => {
                println!("ADD L");
            }
            0x86 => {
                println!("ADD M");
            }
            0x87 => {
                println!("ADD A");
            }
            0x88 => {
                println!("ADC B");
            }
            0x89 => {
                println!("ADC C");
            }
            0x8a => {
                println!("ADC D");
            }
            0x8b => {
                println!("ADC E");
            }
            0x8c => {
                println!("ADC H");
            }
            0x8d => {
                println!("ADC L");
            }
            0x8e => {
                println!("ADC M");
            }
            0x8f => {
                println!("ADC A");
            }
            0x90 => {
                println!("SUB B");
            }
            0x91 => {
                println!("SUB C");
            }
            0x92 => {
                println!("SUB D");
            }
            0x93 => {
                println!("SUB E");
            }
            0x94 => {
                println!("SUB H");
            }
            0x95 => {
                println!("SUB L");
            }
            0x96 => {
                println!("SUB M");
            }
            0x97 => {
                println!("SUB A");
            }
            0x98 => {
                println!("SBB B");
            }
            0x99 => {
                println!("SBB C");
            }
            0x9a => {
                println!("SBB D");
            }
            0x9b => {
                println!("SBB E");
            }
            0x9c => {
                println!("SBB H");
            }
            0x9d => {
                println!("SBB L");
            }
            0x9e => {
                println!("SBB M");
            }
            0x9f => {
                println!("SBB A");
            }
            0xa0 => {
                println!("ANA B");
            }
            0xa1 => {
                println!("ANA C");
            }
            0xa2 => {
                println!("ANA D");
            }
            0xa3 => {
                println!("ANA E");
            }
            0xa4 => {
                println!("ANA H");
            }
            0xa5 => {
                println!("ANA L");
            }
            0xa6 => {
                println!("ANA M");
            }
            0xa7 => {
                println!("ANA A");
            }
            0xa8 => {
                println!("XRA B");
            }
            0xa9 => {
                println!("XRA C");
            }
            0xaa => {
                println!("XRA D");
            }
            0xab => {
                println!("XRA E");
            }
            0xac => {
                println!("XRA H");
            }
            0xad => {
                println!("XRA L");
            }
            0xae => {
                println!("XRA M");
            }
            0xaf => {
                println!("XRA A");
            }
            0xb0 => {
                println!("ORA B");
            }
            0xb1 => {
                println!("ORA C");
            }
            0xb2 => {
                println!("ORA D");
            }
            0xb3 => {
                println!("ORA E");
            }
            0xb4 => {
                println!("ORA H");
            }
            0xb5 => {
                println!("ORA L");
            }
            0xb6 => {
                println!("ORA M");
            }
            0xb7 => {
                println!("ORA A");
            }
            0xb8 => {
                println!("CMP B");
            }
            0xb9 => {
                println!("CMP C");
            }
            0xba => {
                println!("CMP D");
            }
            0xbb => {
                println!("CMP E");
            }
            0xbc => {
                println!("CMP H");
            }
            0xbd => {
                println!("CMP L");
            }
            0xbe => {
                println!("CMP M");
            }
            0xbf => {
                println!("CMP A");
            }
            0xc0 => {
                println!("RNZ");
            }
            0xc1 => {
                println!("POP B");
            }
            0xc2 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JNZ ${:02X}{:02X}", byte3, byte2);
            }
            0xc3 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JMP ${:02X}{:02X}", byte3, byte2);
            }
            0xc4 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CNZ ${:02X}{:02X}", byte3, byte2);
            }
            0xc5 => {
                println!("PUSH B");
            }
            0xc6 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("ADI #{:02X}", byte2);
            }
            0xc7 => {
                println!("RST 0");
            }
            0xc8 => {
                println!("RZ");
            }
            0xc9 => {
                println!("RET");
            }
            0xca => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JZ ${:02X}{:02X}", byte3, byte2);
            }
            0xcb => {
                println!("NOP");
            }
            0xcc => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CZ ${:02X}{:02X}", byte3, byte2);
            }
            0xcd => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CALL ${:02X}{:02X}", byte3, byte2);
            }
            0xce => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("ACI #{:02X}", byte2);
            }
            0xcf => {
                println!("RST");
            }
            0xd0 => {
                println!("RNC");
            }
            0xd1 => {
                println!("POP D");
            }
            0xd2 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JNC ${:02X}{:02X}", byte3, byte2);
            }
            0xd3 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("OUT #{:02X}", byte2);
            }
            0xd4 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CNC ${:02X}{:02X}", byte3, byte2);
            }
            0xd5 => {
                println!("PUSH D");
            }
            0xd6 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("SUI #{:03X}", byte2);
            }
            0xd7 => {
                println!("RST 2");
            }
            0xd8 => {
                println!("RC");
            }
            0xd9 => {
                println!("NOP");
            }
            0xda => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JC ${:02X}{:02X}", byte3, byte2);
            }
            0xdb => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("IN #{:03X}", byte2);
            }
            0xdc => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CC ${:02X}{:02X}", byte3, byte2);
            }
            0xdd => {
                println!("NOP");
            }
            0xde => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("SBI #{:03X}", byte2);
            }
            0xdf => {
                println!("RST 3");
            }
            0xe0 => {
                println!("RPO");
            }
            0xe1 => {
                println!("POP H");
            }
            0xe2 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JPO ${:02X}{:02X}", byte3, byte2);
            }
            0xe3 => {
                println!("XTHL");
            }
            0xe4 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CPO ${:02X}{:02X}", byte3, byte2);
            }
            0xe5 => {
                println!("PUSH H");
            }
            0xe6 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("ANI #{:03X}", byte2);
            }
            0xe7 => {
                println!("RST 4");
            }
            0xe8 => {
                println!("RPE");
            }
            0xe9 => {
                println!("PCHL");
            }
            0xea => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JPE ${:02X}{:02X}", byte3, byte2);
            }
            0xeb => {
                println!("XCHG");
            }
            0xec => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CPE ${:02X}{:02X}", byte3, byte2);
            }
            0xed => {
                println!("NOP");
            }
            0xee => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("XRI #{:03X}", byte2);
            }
            0xef => {
                println!("RST 5");
            }
            0xf0 => {
                println!("RP");
            }
            0xf1 => {
                println!("POP PSW");
            }
            0xf2 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JP ${:02X}{:02X}", byte3, byte2);
            }
            0xf3 => {
                println!("DI");
            }
            0xf4 => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CP ${:02X}{:02X}", byte3, byte2);
            }
            0xf5 => {
                println!("PUSH PSW");
            }
            0xf6 => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("ORI #{:03X}", byte2);
            }
            0xf7 => {
                println!("RST 6");
            }
            0xf8 => {
                println!("RM");
            }
            0xf9 => {
                println!("SPHL");
            }
            0xfa => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("JM ${:02X}{:02X}", byte3, byte2);
            }
            0xfb => {
                println!("EI");
            }
            0xfc => {
                op_bytes = 3;
                let byte2 = self.memory[(self.pc as usize) + 1];
                let byte3 = self.memory[(self.pc as usize) + 2];
                println!("CM ${:02X}{:02X}", byte3, byte2);
            }
            0xfd => {
                println!("NOP");
            }
            0xfe => {
                op_bytes = 2;
                let byte2 = self.memory[(self.pc as usize) + 1];
                println!("CPI #{:03X}", byte2);
            }
            0xff => {
                println!("RST 7");
            }
            _ => unimplemented!("Unimplemented opcode: {:#02X}", op),
        }
        op_bytes
    }
}
