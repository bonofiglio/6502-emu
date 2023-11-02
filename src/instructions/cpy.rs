use crate::{
    addressing::{absolute, immediate, zeropage},
    compare::cmp,
    cpu::CPU,
};

pub fn cpy_immediate(cpu: &mut CPU) {
    let value = immediate(cpu);

    cmp(cpu, cpu.y, value);
    cpu.program_counter += 2;
}

pub fn cpy_zeropage(cpu: &mut CPU) {
    let value = cpu.memory[zeropage(cpu) as usize];

    cmp(cpu, cpu.y, value);
    cpu.program_counter += 2;
}

pub fn cpy_absolute(cpu: &mut CPU) {
    let value = cpu.memory[absolute(cpu) as usize];

    cmp(cpu, cpu.y, value);
    cpu.program_counter += 3;
}
