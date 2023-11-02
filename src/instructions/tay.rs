use crate::{cpu::CPU, flags::set_nz};

pub fn tay(cpu: &mut CPU) {
    cpu.y = cpu.accumulator;
    set_nz(cpu, cpu.y);
    cpu.program_counter += 1;
}
