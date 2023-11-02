use crate::{cpu::CPU, flags::clear_v};

pub fn clv(cpu: &mut CPU) {
    cpu.program_counter += 1;
    clear_v(cpu)
}
