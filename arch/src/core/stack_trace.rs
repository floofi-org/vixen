use core::fmt::{Display, Formatter};
use crate::core::Interrupt;
use crate::CPU;
use crate::cpu::Decoder;

pub struct StackTrace<'a> {
    cpu: &'a CPU,
    interrupt: Interrupt
}

impl<'a> StackTrace<'a> {
    #[must_use]
    pub fn new(interrupt: Interrupt, cpu: &'a CPU) -> Self {
        Self {
            cpu,
            interrupt
        }
    }
}

impl Display for StackTrace<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let cpu = self.cpu;
        let stack_trace = Interrupt::get_stack_trace(&cpu.system_stack, cpu.status_register);

        write!(f, include!("stack_trace_template.txt"),
               reason = match (cpu.status_register.interrupt, cpu.status_register.double_fault) {
                (false, false) => "interrupt",
                (true, _) => "double fault",
                (_, true) => "triple fault"
            },
               interrupt = self.interrupt,
               r0 = cpu.registers.r0,
               r1 = cpu.registers.r1,
               r2 = cpu.registers.r2,
               r3 = cpu.registers.r3,
               r4 = cpu.registers.r4,
               r5 = cpu.registers.r5,
               r6 = cpu.registers.r6,
               r7 = cpu.registers.r7,
               r8 = cpu.registers.r8,
               r9 = cpu.registers.r9,
               r10 = cpu.registers.r10,
               r11 = cpu.registers.r11,
               r12 = cpu.registers.r12,
               r13 = cpu.registers.r13,
               r14 = cpu.registers.r14,
               sp = cpu.stack_pointer,
               sr = cpu.status_register,
               pc = cpu.program_counter,
               state = cpu.extract_instruction_infailible(cpu.program_counter),
               disassembler = cpu.read_instruction_string(cpu.program_counter),
               stack_trace = stack_trace,
        )
    }
}
