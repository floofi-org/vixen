mod commands;

use std::ffi::OsString;
use std::process::exit;
use std::{env, fs, io};
use std::io::Write;
use vixen::core::Interrupt;
use vixen::{BusDevice, CPU, MEMORY_64M};
use vixen::cpu::Decoder;
use vixen::CPUResult;
use vixen_devices::{RealTimeClock, Terminal};

use stdin::DebuggerStdin;

mod stdin;

struct DebuggerState {
    pub running: bool,
    pub interrupt: Option<Interrupt>,
    pub stdin: DebuggerStdin,
}

fn main() {
    let path = get_rom_path().unwrap_or_else(|| {
        eprintln!("\u{1b}[33mUsage: vdbg {{rom}}\u{1b}[0m");
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

    let stdin = DebuggerStdin::new();

    let devices: Vec<Box<dyn BusDevice>> = vec![
        Box::new(Terminal::new(stdin.clone())),
        Box::new(RealTimeClock::now())
    ];

    if let Err(e) = cpu.register_devices(devices) {
        eprintln!("\u{1b}[33mFailed to start up devices: {e}\u{1b}[0m");
        exit(2);
    }

    let mut state = DebuggerState {
        running: false,
        interrupt: None,
        stdin,
    };

    debug_cpu(&mut state, &mut cpu, rom.len());
}

fn get_rom_path() -> Option<OsString> {
    // Skip binary path
    env::args_os().nth(1)
}

fn debugger_prompt(cpu: &mut CPU, state: &mut DebuggerState) -> CPUResult<()> {
    if state.running {
        cpu.tick()?;
        cpu.program_counter += 15;
        return Ok(());
    }

    print!("\u{1b}[33m(vdbg)\u{1b}[0m ");
    let _ = io::stdout().flush();

    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let line = line.trim();

    match line {
        "?" | "help" => commands::help(),
        "s" | "step" => commands::step(state, cpu)?,
        "b" | "unblock" => commands::unblock(state, cpu),
        "j" | "jump" => commands::jump(cpu),
        "r" | "run" => commands::run(state),
        "l" | "location" => commands::location(cpu),
        "g" | "registers" => commands::registers(cpu),
        "i" | "interrupt" => commands::interrupt(state, cpu),
        "e" | "expand" => commands::expand(cpu),
        "q" | "quit" => commands::quit(),
        line if line.starts_with('>') || line.starts_with("input") => commands::input(state, line),
        _ => commands::default(cpu, line)
    }

    Ok(())
}

fn debug_cpu(state: &mut DebuggerState, cpu: &mut CPU, rom_size: usize) {
    println!("\u{1b}[33mLoaded {rom_size} bytes of system ROM.\u{1b}[0m");
    println!("\u{1b}[33mProgram at {:0>8x}: {}\u{1b}[0m",
             cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));

    loop {
        if let Err(interrupt) = debugger_prompt(cpu, state) {
            state.interrupt = Some(interrupt);
            state.running = false;
            println!("\u{1b}[33mUnhandled interrupt {interrupt} at {:0>8x}: {}\u{1b}[0m",
                     cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));
        }
    }
}
