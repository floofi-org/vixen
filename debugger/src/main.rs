use std::ffi::OsString;
use std::process::exit;
use std::{env, fs, io};
use std::io::Write;
use vixen::core::Interrupt;
use vixen::core::StackTrace;
use vixen::{CPU, MEMORY_64M};
use vixen::cpu::Decoder;
use vixen::CPUResult;

#[derive(Default)]
struct DebuggerState {
    pub running: bool,
    pub interrupt: Option<Interrupt>
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

    debug_cpu(&mut cpu, rom.len());
}

fn get_rom_path() -> Option<OsString> {
    // Skip binary path
    env::args_os().nth(1)
}

fn dump_memory(cpu: &mut CPU, start: usize, end: usize, focus: Option<usize>) {
    let start = start.max(0);
    let end = end.min(0xffff_ffff);
    let mut position = start;

    while position < end {
        print!("\u{1b}[0m{position:0>8x}:  ");
        for _ in 0..16 {
            let focus_start = focus.unwrap_or(cpu.program_counter as usize);
            let focus_end = focus_start + if focus.is_some() { 1 } else { 10 };
            match position {
                x if x > 0xffff_fffe => (),
                x if x == focus_end - 1 => print!("\u{1b}[43m{:0>2x}\u{1b}[0m ", cpu.memory[position]),
                x if (focus_start..focus_end).contains(&x) => print!("\u{1b}[43m{:0>2x} ", cpu.memory[position]),
                _ => print!("{:0>2x} ", cpu.memory[position])
            }
            position += 1;
        }
        println!();
    }
}

fn debugger_prompt(cpu: &mut CPU, state: &mut DebuggerState) -> CPUResult<()> {
    if state.running {
        cpu.tick()?;
        cpu.program_counter += 10;
        return Ok(());
    }

    print!("\u{1b}[33m(vdbg)\u{1b}[0m ");
    let _ = io::stdout().flush();

    let mut line = String::new();
    let _ = io::stdin().read_line(&mut line);
    let line = line.trim();

    match line {
        "?" | "help" => {
            println!("Debugger commands:");
            println!("  step         -- Run a single clock cycle (shorthand: s)");
            println!("  run          -- Run system until breakpoint is hit (shorthand: r)");
            println!("  help         -- Show this message (shorthand: ?)");
            println!("  quit         -- Abort program (shorthand: q)");
            println!("  unblock      -- Unblock interrupted system (shorthand: b)");
            println!("  interrupt    -- Display interrupt stack trace (shorthand: i)");
            println!("  registers    -- Display current system registers (shorthand: g)");
            println!("  location     -- Show program location in memory (shorthand: l)");
            println!("  <hex addr>   -- Display memory address");
        },
        "s" | "step" => {
            if state.interrupt.is_some() {
                println!("\u{1b}[33mSystem blocked on interrupt. 'i' for stack trace, 'b' to resume.\u{1b}[0m");
            } else {
                cpu.tick()?;
                cpu.program_counter += 10;
                println!("\u{1b}[33mProgram at {:0>8x}: {}\u{1b}[0m",
                         cpu.program_counter, cpu.read_instruction_string(cpu.program_counter, false));
            }
        },
        "b" | "unblock" => {
            state.interrupt = None;
            cpu.program_counter += 10;
            println!("\u{1b}[33mSystem unblocked. Ignoring interrupts is unsafe, you are on your own.\u{1b}[0m");
        },
        "r" | "run" => {
            if state.interrupt.is_some() {
                println!("\u{1b}[33mSystem blocked on interrupt. 'i' for stack trace, 'b' to resume.\u{1b}[0m");
            } else {
                state.running = true;
            }
        },
        // We want to get a valid memory address at the end, this is intended
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        "l" | "location" => {
            let start = (f64::from(cpu.program_counter) / 32.0).round() as usize * 32 - 32;
            let end = (f64::from(cpu.program_counter) / 32.0).round() as usize * 32 + 32;
            dump_memory(cpu, start, end, None);
        },
        "g" | "registers" => {
            println!("r0  = {register1:0>8x}, r1  = {register2:0>8x}, r2  = {register3:0>8x}",
                     register1 = cpu.registers.r0, register2 = cpu.registers.r1, register3 = cpu.registers.r2);
            println!("r3  = {register1:0>8x}, r4  = {register2:0>8x}, r5  = {register3:0>8x}",
                     register1 = cpu.registers.r3, register2 = cpu.registers.r4, register3 = cpu.registers.r5);
            println!("r6  = {register1:0>8x}, r7  = {register2:0>8x}, r8  = {register3:0>8x}",
                     register1 = cpu.registers.r6, register2 = cpu.registers.r7, register3 = cpu.registers.r8);
            println!("r9  = {register1:0>8x}, r10 = {register2:0>8x}, r11 = {register3:0>8x}",
                     register1 = cpu.registers.r9, register2 = cpu.registers.r10, register3 = cpu.registers.r11);
            println!("r12 = {register1:0>8x}, r13 = {register2:0>8x}, r14 = {register3:0>8x}",
                     register1 = cpu.registers.r12, register2 = cpu.registers.r13, register3 = cpu.registers.r14);
        },
        "i" | "interrupt" => {
            if state.interrupt.is_some() {
                println!("{}", StackTrace::new(state.interrupt.unwrap(), cpu));
            } else {
                println!("\u{1b}[33mSystem is not blocked.\u{1b}[0m");
            }
        },
        "q" | "quit" => {
            exit(0);
        },
        _ => {
            if let Ok(number) = u16::from_str_radix(line, 16) {
                // We want to get a valid memory address at the end, this is intended
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                if number == 0xFFFF {
                    println!("\u{1b}[33mInvalid memory address.\u{1b}[0m");
                } else {
                    let start = (f32::from(number.clamp(32, 65503)) / 32.0).round() as usize * 32 - 32;
                    let end = (f32::from(number.clamp(32, 65503)) / 32.0).round() as usize * 32 + 32;
                    dump_memory(cpu, start, end, Some(number as usize));
                }
            } else {
                println!("\u{1b}[33mInvalid or empty command.\u{1b}[0m");
            }
        }
    }

    Ok(())
}

fn debug_cpu(cpu: &mut CPU, rom_size: usize) {
    println!("\u{1b}[33mLoaded {rom_size} bytes of system ROM.\u{1b}[0m");
    println!("\u{1b}[33mProgram at {:0>8x}: {}\u{1b}[0m",
             cpu.program_counter, cpu.read_instruction_string(cpu.program_counter, false));
    let mut state = DebuggerState::default();
    loop {
        if let Err(interrupt) = debugger_prompt(cpu, &mut state) {
            state.interrupt = Some(interrupt);
            state.running = false;
            println!("\u{1b}[33mUnhandled interrupt {interrupt} at {:0>8x}: {}\u{1b}[0m",
                     cpu.program_counter, cpu.read_instruction_string(cpu.program_counter, false));
        }
    }
}
