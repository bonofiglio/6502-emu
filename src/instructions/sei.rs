use crate::{cpu::CPU, flags::set_ip};

pub fn sei(cpu: &mut CPU) {
    set_ip(cpu);
    cpu.program_counter += 1;
}
