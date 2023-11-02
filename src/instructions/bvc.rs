use crate::{addressing::relative, cpu::CPU, flags::get_v};

pub fn bvc(cpu: &mut CPU) {
    if !get_v(cpu) {
        cpu.program_counter = cpu
            .program_counter
            .wrapping_sub(relative(cpu).wrapping_neg() as u16)
            + 2;
    } else {
        cpu.program_counter += 2;
    }
}
