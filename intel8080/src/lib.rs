const NUM_REGS: usize = 7;
const MEMORY_SIZE: usize = 65_536;
pub struct Intel8080 {
    regs: [u8; NUM_REGS],
    memory: [u8; MEMORY_SIZE],
    pc: u16,
    sp: u16,
}
