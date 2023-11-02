use crate::{cpu::CPU, flags::set_c};

pub fn sec(cpu: &mut CPU) {
    set_c(cpu);
    cpu.program_counter += 1;
}
