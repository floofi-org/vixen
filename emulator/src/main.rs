use std::ffi::OsString;
use std::process::exit;
use std::{env, fs};

use vixen::core::Interrupt;
use vixen::core::StackTrace;
use vixen::{BusDevice, CPU, MEMORY_64M};
use vixen::CPUResult;
use vixen_devices::terminal::TerminalDevice;

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

    if rom.len() > 67_108_864 {
        eprintln!("\u{1b}[33mROM is too large ({} bytes) for the reserved memory space \
        (67108864 bytes).\u{1b}[0m", rom.len());
        exit(2);
    }

    let mut cpu = CPU::new(MEMORY_64M);
    if let Err(e) = cpu.load_rom(&rom) {
        eprintln!("\u{1b}[33mFailed to load ROM into CPU: {e}\u{1b}[0m");
        exit(2);
    }
    
    let devices: Vec<Box<dyn BusDevice>> = vec![
        Box::new(TerminalDevice::new())
    ];
    if let Err(e) = cpu.register_devices(devices) {
        eprintln!("\u{1b}[33mFailed to start up devices: {e}\u{1b}[0m");
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
        cpu.program_counter += 15;
    }
}

fn on_unhandled_interrupt(cpu: &CPU, interrupt: Interrupt) {
    println!("\u{1b}[33m{}\u{1b}[0m", StackTrace::new(interrupt, cpu));
}
