use std::fs;
use vixen::cpu::CPU;

fn main() {
    let file = fs::read("./rom.bin").unwrap();
    let mut cpu = CPU::default();
    cpu.load_rom(&file);

    if let Some(stack_trace) = cpu.run() {
        println!("{}", stack_trace);
    }
}
