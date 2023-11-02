use crate::{cpu::CPU, flags::set_dp};

pub fn sed(cpu: &mut CPU) {
    set_dp(cpu);
    cpu.program_counter += 1;
}
