use std::fs;
use dumbcpu::{CPU, CPUResult, Interrupt};

fn run_cpu(cpu: &mut CPU) -> CPUResult<Interrupt> {
    loop {
        let pc = cpu.pc;
        let instruction =  cpu.read_instruction(pc)?;
        println!("{:#x?}", &instruction);
        cpu.pc += 6;
    }
}

fn main() {
    let file = fs::read("./rom.bin").unwrap();
    let mut cpu = CPU::new(&file);

    if let Err(interrupt) = run_cpu(&mut cpu) {
        println!();
        println!("Unhandled interrupt: {interrupt}");

        println!();
        println!("a = {a:0>2X}; x = {x:0>2X}; y = {y:0>2X}",
                 a = cpu.registers.a, x = cpu.registers.x, y = cpu.registers.y);
        println!("r0 = {r0:0>2X}; r1 = {r1:0>2X}; r2 = {r2:0>2X}; r3 = {r3:0>2X}; r4 = {r4:0>2X}; \
        r5 = {r5:0>2X}; r6 = {r6:0>2X}; r7 = {r7:0>2X}",
                 r0 = cpu.registers.r0, r1 = cpu.registers.r1, r2 = cpu.registers.r2,
                 r3 = cpu.registers.r3, r4 = cpu.registers.r4, r5 = cpu.registers.r5,
                 r6 = cpu.registers.r6, r7 = cpu.registers.r7);
        println!("sp = {sp:0>2X}; sr = {sr}",
                 sp = cpu.sp, sr = cpu.sr);

        println!();
        println!("pc = {pc:0>4X}: {state}: <disassembler not implemented>", pc = cpu.pc, state = cpu.extract_instruction(cpu.pc));

        println!();
        print!("zerop = ");

        for (index, byte) in cpu.memory[0x0000..0x00FF].iter().enumerate() {
            print!("{:0>2X}", byte);
            if index % 1 == 0 {
                print!(" ");
            }
            if (index + 1) % 32 == 0 {
                print!("\n        ");
            }
        }

        println!();
        print!("\nstack = ");

        for (index, byte) in cpu.memory[0x0100..0x01FF].iter().enumerate() {
            print!("{:0>2X}", byte);
            if index % 1 == 0 {
                print!(" ");
            }
            if (index + 1) % 32 == 0 {
                print!("\n        ");
            }
        }

        println!("\n            - _interrupt (????): <not handled>");
        println!("            - <stack unwinding not implemented>");
    }
}
