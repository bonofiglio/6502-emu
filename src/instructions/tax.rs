use crate::{cpu::CPU, flags::set_nz};

pub fn tax(cpu: &mut CPU) {
    cpu.x = cpu.accumulator;
    set_nz(cpu, cpu.x);
    cpu.program_counter += 1;
}
