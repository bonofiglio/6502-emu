use crate::{addressing::absolute, cpu::CPU};

pub fn jsr(cpu: &mut CPU) {
    cpu.stack_push((cpu.program_counter >> 8) as u8);
    cpu.stack_push((cpu.program_counter & 0xFF) as u8);

    cpu.program_counter = absolute(cpu)
}
