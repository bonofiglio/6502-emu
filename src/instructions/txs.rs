use crate::{cpu::CPU, flags::set_nz};

pub fn txs(cpu: &mut CPU) {
    cpu.stack_pointer = cpu.x;
    set_nz(cpu, cpu.stack_pointer);
    cpu.program_counter += 1;
}
