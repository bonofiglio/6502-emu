use crate::{addressing::absolute, cpu::CPU};

pub fn jsr(cpu: &mut CPU) {
    let pc = cpu.program_counter + 2;

    cpu.stack_push((pc >> 8) as u8);
    cpu.stack_push((pc & 0xFF) as u8);

    cpu.program_counter = absolute(cpu)
}
