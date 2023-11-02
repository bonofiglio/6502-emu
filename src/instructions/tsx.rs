use crate::{cpu::CPU, flags::set_nz};

pub fn tsx(cpu: &mut CPU) {
    cpu.x = cpu.stack_pointer;
    set_nz(cpu, cpu.x);
    cpu.program_counter += 1;
}
