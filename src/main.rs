mod cpu;
mod opcode;
mod port;
mod register;
mod rom;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    cpu.load_file("sample.bin");
    println!("{:4} {}", "code", "output");
    for i in 0..100 {
        cpu.execute();
        println!("{:<04b} {:>04b}", i, cpu.port.output);
    }
}
