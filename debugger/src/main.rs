use std::ffi::OsString;
use std::process::exit;
use std::{env, fs, io};
use std::io::Write;
use vixen::core::Interrupt;
use vixen::core::StackTrace;
use vixen::CPU;
use vixen::cpu::Decoder;
use vixen::CPUResult;

#[derive(Default)]
struct DebuggerState {
    pub running: bool,
    pub interrupt: Option<Interrupt>
}

fn main() {
    let path = get_rom_path().unwrap_or_else(|| {
        eprintln!("\u{1b}[33mUsage: vxdbg {{rom}}\u{1b}[0m");
        eprintln!("\u{1b}[33mPlease provide path to ROM.\u{1b}[0m");

        exit(-1);
    });

    let rom = fs::read(path);
    let rom = rom.unwrap_or_else(|e| {
        eprintln!("\u{1b}[33mFailed to read ROM file: {e}\u{1b}[0m");
        exit(-1);
    });
    let mut cpu = CPU::default();
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
    let end = end.min(65535);
    let mut position = start;

    while position < end {
        print!("\u{1b}[0m{position:0>4X}:  ");
        for _ in 0..16 {
            let focus_start = focus.unwrap_or(cpu.program_counter as usize);
            let focus_end = focus_start + if focus.is_some() { 1 } else { 6 };
            match position {
                x if x > 65534 => (),
                x if x == focus_end - 1 => print!("\u{1b}[43m{:0>2X}\u{1b}[0m ", cpu.memory[position]),
                x if (focus_start..focus_end).contains(&x) => print!("\u{1b}[43m{:0>2X} ", cpu.memory[position]),
                _ => print!("{:0>2X} ", cpu.memory[position])
            }
            position += 1;
        }
        println!();
    }
}

fn debugger_prompt(cpu: &mut CPU, state: &mut DebuggerState) -> CPUResult<()> {
    if state.running {
        cpu.tick()?;
        cpu.program_counter += 6;
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
                cpu.program_counter += 6;
                println!("\u{1b}[33mProgram at {:0>4X}: {}\u{1b}[0m",
                         cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));
            }
        },
        "b" | "unblock" => {
            state.interrupt = None;
            cpu.program_counter += 6;
            println!("\u{1b}[33mSystem unblocked. Ignoring interrupts is unsafe, you are on your own.\u{1b}[0m");
        },
        "r" | "run" => {
            state.running = true;
        },
        // We want to get a valid memory address at the end, this is intended
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        "l" | "location" => {
            let start = (f32::from(cpu.program_counter) / 32.0).round() as usize * 32 - 32;
            let end = (f32::from(cpu.program_counter) / 32.0).round() as usize * 32 + 32;
            dump_memory(cpu, start, end, None);
        },
        "g" | "registers" => {
            println!("A  = 0x{register:0>4X}, {register}", register = cpu.registers.a);
            println!("X  = 0x{register:0>4X}, {register}", register = cpu.registers.x);
            println!("Y  = 0x{register:0>4X}, {register}", register = cpu.registers.y);
            println!("R0 = 0x{register:0>4X}, {register}", register = cpu.registers.r0);
            println!("R1 = 0x{register:0>4X}, {register}", register = cpu.registers.r1);
            println!("R2 = 0x{register:0>4X}, {register}", register = cpu.registers.r2);
            println!("R3 = 0x{register:0>4X}, {register}", register = cpu.registers.r3);
            println!("R4 = 0x{register:0>4X}, {register}", register = cpu.registers.r4);
            println!("R5 = 0x{register:0>4X}, {register}", register = cpu.registers.r5);
            println!("R6 = 0x{register:0>4X}, {register}", register = cpu.registers.r6);
            println!("R7 = 0x{register:0>4X}, {register}", register = cpu.registers.r7);
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
    println!("\u{1b}[33mProgram at {:0>4X}: {}\u{1b}[0m",
             cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));
    let mut state = DebuggerState::default();
    loop {
        if let Err(interrupt) = debugger_prompt(cpu, &mut state) {
            state.interrupt = Some(interrupt);
            state.running = false;
            println!("\u{1b}[33mUnhandled interrupt {interrupt} at {:0>4X}: {}\u{1b}[0m",
                     cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));
        }
    }
}
