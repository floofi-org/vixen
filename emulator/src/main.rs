use std::ffi::OsString;
use std::process::exit;
use std::{env, fs};

use vixen::core::Interrupt;
use vixen::core::StackTrace;
use vixen::CPU;
use vixen::CPUResult;

fn main() {
    let path = get_rom_path().unwrap_or_else(|| {
        eprintln!("\u{1b}[33mUsage: vxemu {{rom}}\u{1b}[0m");
        eprintln!("\u{1b}[33mPlease provide path to ROM.\u{1b}[0m");

        exit(-1);
    });

    let rom = fs::read(path);
    let rom = rom.unwrap_or_else(|e| {
        eprintln!("\u{1b}[33mFailed to read ROM file: {e}\u{1b}[0m");
        exit(-1);
    });

    if rom.len() > 33_553_920 {
        eprintln!("\u{1b}[33mROM is too large ({} bytes) for the reserved memory space \
        (33553920 bytes).\u{1b}[0m", rom.len());
        exit(2);
    }

    let mut cpu = CPU::default();
    if let Err(e) = cpu.load_rom(&rom) {
        eprintln!("\u{1b}[33mFailed to load ROM into CPU: {e}\u{1b}[0m");
        exit(2);
    }

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
        cpu.program_counter += 10;
    }
}

fn on_unhandled_interrupt(cpu: &CPU, interrupt: Interrupt) {
    println!("\u{1b}[33m{}\u{1b}[0m", StackTrace::new(interrupt, cpu));

    let result = fs::write("./memory.bin", &cpu.memory);
    match result {
        Ok(()) => println!("\u{1b}[33mCore dumped to 'memory.bin'.\u{1b}[0m"),
        Err(e) => println!("\u{1b}[33mFailed to dump memory: {e}\u{1b}[0m"),
    }
}
