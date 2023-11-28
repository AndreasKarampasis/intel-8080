use intel8080::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
fn main() {
    let args: Vec<_> = env::args().collect();
    let mut cpu: Intel8080 = Intel8080::new();
    let mut rom: File = File::open(&args[1]).expect("Unable to open file");
    let mut buffer: Vec<u8> = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    cpu.load(&buffer);
    let mut guess = String::new();
    let mut i = 0;
    while cpu.get_pc() < (buffer.len() as u16) {
        /*
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            */
        i += 1;
        cpu.print_state();
        cpu.tick();
        println!("{}", i);
    }
}
