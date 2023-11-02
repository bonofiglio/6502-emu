use crate::{
    addressing::{absolute, indirect},
    cpu::CPU,
};

pub fn jmp_absolute(cpu: &mut CPU) {
    cpu.program_counter = absolute(cpu);
}

pub fn jmp_indirect(cpu: &mut CPU) {
    cpu.program_counter = indirect(cpu);
}
