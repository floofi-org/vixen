use vixen::CPU;

pub fn dump_memory(cpu: &mut CPU, start: usize, end: usize, focus: Option<usize>) {
    let start = start.max(0);
    let end = end.min(0xffff_ffff);
    let mut position = start;

    while position < end {
        print!("\u{1b}[0m{position:0>8x}:  ");
        for _ in 0..16 {
            let focus_start = focus.unwrap_or(cpu.program_counter as usize);
            let focus_end = focus_start + if focus.is_some() { 4 } else { 15 };
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
