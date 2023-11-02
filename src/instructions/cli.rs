use crate::{cpu::CPU, flags::clear_ip};

pub fn cli(cpu: &mut CPU) {
    cpu.program_counter += 1;
    clear_ip(cpu)
}
