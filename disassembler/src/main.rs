use std::ffi::OsString;
use std::process::exit;
use std::{env, fs};
use vixen::{CPU, MEMORY_NONE};
use vixen::cpu::Decoder;

fn main() {
    let path = get_rom_path().unwrap_or_else(|| {
        eprintln!("\u{1b}[33mUsage: vdas {{rom}}\u{1b}[0m");
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

    let mut cpu = CPU::new(MEMORY_NONE);
    if let Err(e) = cpu.load_rom(&rom) {
        eprintln!("\u{1b}[33mFailed to load ROM into CPU: {e}\u{1b}[0m");
        exit(2);
    }

    let disassembled = disassemble_rom(cpu, &rom);
    let disassembled = disassembled.trim_end();
    println!("{disassembled}");
}

#[allow(clippy::cast_possible_truncation)]
fn disassemble_rom(mut cpu: CPU, rom: &Vec<u8>) -> String {
    let mut disassembled = String::new();

    while cpu.program_counter < (cpu.memory.len() - 1) as u32 && cpu.program_counter < rom.len() as u32 + 0x1ff {
        let text = cpu.read_instruction_string(cpu.program_counter, true);
        disassembled.push_str(&text);
        disassembled.push('\n');
        cpu.program_counter += 15;
    }

    disassembled
}

fn get_rom_path() -> Option<OsString> {
    // Skip binary path
    env::args_os().nth(1)
}
