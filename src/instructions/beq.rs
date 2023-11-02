use crate::{addressing::relative, cpu::CPU, flags::get_z};

pub fn beq(cpu: &mut CPU) {
    if get_z(cpu) {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_sub(relative(cpu).wrapping_neg() as u16)
            + 2;
    } else {
        cpu.program_counter += 2;
    }
}
