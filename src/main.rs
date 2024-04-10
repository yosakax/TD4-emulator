mod cpu;
mod opcode;
mod port;
mod register;
mod rom;

use cpu::Cpu;

fn main() {
    let mut cpu = Cpu::new();
    // cpu.load_file("code/sample.bin");
    // cpu.load_file("code/Lchika.bin");
    cpu.load_file("code/ramen_timer.bin");
    println!("{:4} {}", "pc", "output");
    let mut is_last = false;
    for i in 0..1000 {
        cpu.execute();
        println!("{:<04} {:>04b} {}", cpu.pc, cpu.port.output, cpu.carry);
        if cpu.pc == 15 {
            if is_last {
                break;
            } else {
                is_last = true;
            }
        }
    }
}
