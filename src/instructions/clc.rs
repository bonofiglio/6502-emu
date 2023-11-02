use crate::{cpu::CPU, flags::clear_c};

pub fn clc(cpu: &mut CPU) {
    cpu.program_counter += 1;
    clear_c(cpu);
}
