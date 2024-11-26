use std::fs;
use dumbcpu::CPUResult;
use dumbcpu::core::interrupt::Interrupt;
use dumbcpu::cpu::CPU;
use dumbcpu::cpu::decoder::Decoder;

fn run_cpu(cpu: &mut CPU) -> CPUResult<Interrupt> {
    loop {
        let pc = cpu.pc;
        let instruction =  cpu.read_instruction(pc)?;
        instruction.execute(cpu)?;
        cpu.pc += 6;
    }
}

fn main() {
    let file = fs::read("./rom.bin").unwrap();
    let mut cpu = CPU::default();
    cpu.load_rom(&file);

    if let Err(interrupt) = run_cpu(&mut cpu) {
        interrupt.stack_trace(cpu);
    }
}
