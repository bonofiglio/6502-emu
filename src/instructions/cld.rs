use crate::{cpu::CPU, flags::clear_dp};

pub fn cld(cpu: &mut CPU) {
    cpu.program_counter += 1;
    clear_dp(cpu)
}
