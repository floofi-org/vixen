use std::fs;
use std::process::exit;
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
        let dump_result = fs::write("./memory.bin", cpu.memory);
        println!("{}", StackTrace::new(interrupt, cpu));
        if dump_result.is_err() {
            println!("Failed to dump memory.");
        } else {
            println!("Core dumped to 'memory.bin'.");
        }
        exit(1);
    }
}
