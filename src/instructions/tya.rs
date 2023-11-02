use crate::{cpu::CPU, flags::set_nz};

pub fn tya(cpu: &mut CPU) {
    cpu.accumulator = cpu.y;
    set_nz(cpu, cpu.accumulator);
    cpu.program_counter += 1;
}
