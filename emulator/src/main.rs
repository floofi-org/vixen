use std::fs;
use vixen::core::stack_trace::StackTrace;
use vixen::cpu::CPU;
use vixen::CPUResult;

fn run_cpu(cpu: &mut CPU) -> CPUResult<()> {
    loop {
        cpu.tick()?;
        cpu.program_counter += 6;
    }
}

fn main() {
    let file = fs::read("./rom.bin").unwrap();
    let mut cpu = CPU::default();
    cpu.load_rom(&file);

    if let Err(interrupt) = run_cpu(&mut cpu) {
        println!("{}", StackTrace::new(interrupt, cpu));
    }
}
