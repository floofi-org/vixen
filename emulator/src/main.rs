use std::ffi::OsString;
use std::process::exit;
use std::{env, fs};

use vixen::core::interrupt::Interrupt;
use vixen::core::stack_trace::StackTrace;
use vixen::cpu::CPU;
use vixen::CPUResult;

fn main() {
    let path = get_rom_path().unwrap_or_else(|| {
        eprintln!("Usage: vixen-emulator {{rom}}");
        eprintln!("Please provide path to ROM.");

        exit(-1);
    });

    let rom = fs::read(path);
    let rom = rom.unwrap_or_else(|e| {
        eprintln!("Failed reading ROM file: {e}");
        exit(-1);
    });

    let mut cpu = CPU::default();
    cpu.load_rom(&rom);

    if let Err(interrupt) = run_cpu(&mut cpu) {
        on_unhandled_interrupt(&cpu, interrupt);
        exit(1);
    }
}

fn get_rom_path() -> Option<OsString> {
    // Skip binary path
    env::args_os().nth(1)
}

fn run_cpu(cpu: &mut CPU) -> CPUResult<()> {
    loop {
        cpu.tick()?;
        cpu.program_counter += 6;
    }
}

fn on_unhandled_interrupt(cpu: &CPU, interrupt: Interrupt) {
    println!("{}", StackTrace::new(interrupt, cpu));

    let result = fs::write("./memory.bin", cpu.memory);
    match result {
        Ok(_) => println!("Core dumped to 'memory.bin'."),
        Err(e) => println!("Failed to dump memory: {e}"),
    }
}
