use std::process::exit;
use vdbg::dump_memory;
use vixen::{CPUResult, CPU};
use vixen::core::instruction::{Addressing, Operation};
use vixen::core::{Instruction, MemoryCell, Operand, StackTrace};
use vixen::cpu::Decoder;
use crate::DebuggerState;

pub fn help() {
    println!("Debugger commands:");
    println!("  step         -- Run a single clock cycle (shorthand: s)");
    println!("  jump         -- Jump back an instruction (shorthand: j)");
    println!("  run          -- Run system until breakpoint is hit (shorthand: r)");
    println!("  help         -- Show this message (shorthand: ?)");
    println!("  quit         -- Abort program (shorthand: q)");
    println!("  unblock      -- Unblock interrupted system (shorthand: b)");
    println!("  interrupt    -- Display interrupt stack trace (shorthand: i)");
    println!("  registers    -- Display current system registers (shorthand: g)");
    println!("  location     -- Show program location in memory (shorthand: l)");
    println!("  expand       -- Expand a binary instruction (shorthand: e)");
    println!("  input        -- Write to stdin (shorthand: >)");
    println!("  <hex addr>   -- Display memory address");
}


pub fn step(state: &mut DebuggerState, cpu: &mut CPU) -> CPUResult<()> {
    if state.interrupt.is_some() {
        println!("\u{1b}[33mSystem blocked on interrupt. 'i' for stack trace, 'b' to resume.\u{1b}[0m");
    } else {
        cpu.tick()?;
        cpu.program_counter += 15;
        println!("\u{1b}[33mProgram at {:0>8x}: {}\u{1b}[0m",
                 cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));
    }

    Ok(())
}

pub fn unblock(state: &mut DebuggerState, cpu: &mut CPU) {
    state.interrupt = None;
    cpu.program_counter += 15;
    println!("\u{1b}[33mSystem unblocked. Ignoring interrupts is unsafe, you are on your own.\u{1b}[0m");
}

pub fn jump(cpu: &mut CPU) {
    cpu.program_counter -= 15;
    println!("\u{1b}[33mProgram at {:0>8x}: {}\u{1b}[0m",
             cpu.program_counter, cpu.read_instruction_string(cpu.program_counter));
}

pub fn run(state: &mut DebuggerState) {
    if state.interrupt.is_some() {
        println!("\u{1b}[33mSystem blocked on interrupt. 'i' for stack trace, 'b' to resume.\u{1b}[0m");
    } else {
        state.running = true;
    }
}

// We want to get a valid memory address at the end, this is intended
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn location(cpu: &mut CPU) {
    let start = (f64::from(cpu.program_counter) / 32.0).round() as usize * 32 - 32;
    let end = (f64::from(cpu.program_counter) / 32.0).round() as usize * 32 + 32;
    dump_memory(cpu, start, end, None);
}

pub fn registers(cpu: &mut CPU) {
    println!("sr  = {}", cpu.status_register);
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
}

pub fn interrupt(state: &mut DebuggerState, cpu: &mut CPU) {
    if state.interrupt.is_some() {
        println!("{}", StackTrace::new(state.interrupt.unwrap(), cpu));
    } else {
        println!("\u{1b}[33mSystem is not blocked.\u{1b}[0m");
    }
}

#[allow(clippy::cast_possible_truncation)]
pub fn expand(cpu: &mut CPU) {

    let info = cpu.decode_instruction(cpu.program_counter);
    print!("{:0>3x}: {} @ {:0>8x}", info.operation, Operation::disassemble(info.operation as u16).trim(), cpu.program_counter);
    let parsed: CPUResult<Instruction> = cpu.decode_instruction(cpu.program_counter).try_into();
    if parsed.is_err() {
        print!(" <!>");
    }
    println!();

    for (i, operand) in info.operands.iter().enumerate() {
        let mode = info.modes[i];
        print!("    mem {mode:x}: ");

        if let Ok(decoded_mode) = Addressing::try_from(mode as u8) {
            println!("{decoded_mode:?}");
            print!("        {:0>8x}: {}",
                operand,
                Operand::disassemble(*operand, cpu, decoded_mode)
            );
            if let Addressing::Indirect | Addressing::RegisterIndirect = decoded_mode {
                print!(" -> ");
                match Operand::decode(*operand, cpu, decoded_mode) {
                    Ok(decoded_operand) => match decoded_operand.read_word() {
                        Ok(word) => println!("{word:0>8x}"),
                        Err(_) => println!("?")
                    },
                    Err(_) => println!("?")
                }
            } else {
                println!();
            }
        } else {
            println!("        !?");
        }
    }
}

pub fn input(state: &mut DebuggerState, line: &str) {
    let line = line.trim_start_matches('>').trim();
    state.stdin.write(line);
}

pub fn quit() {
    exit(0);
}

pub fn default(cpu: &mut CPU, line: &str) {
    if let Ok(number) = u32::from_str_radix(line, 16) {
        // We want to get a valid memory address at the end, this is intended
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        if number == 4_294_967_295 {
            println!("\u{1b}[33mInvalid memory address.\u{1b}[0m");
        } else {
            let start = (f64::from(number.clamp(32, 4_294_967_293)) / 32.0).round() as usize * 32 - 32;
            let end = (f64::from(number.clamp(32, 4_294_967_293)) / 32.0).round() as usize * 32 + 32;
            dump_memory(cpu, start, end, Some(number as usize));
        }
    } else {
        println!("\u{1b}[33mInvalid or empty command.\u{1b}[0m");
    }
}
