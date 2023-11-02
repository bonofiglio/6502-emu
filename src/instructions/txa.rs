use crate::{cpu::CPU, flags::set_nz};

pub fn txa(cpu: &mut CPU) {
    cpu.accumulator = cpu.x;
    set_nz(cpu, cpu.accumulator);
    cpu.program_counter += 1;
}
