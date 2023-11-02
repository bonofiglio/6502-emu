use crate::{
    addressing::{absolute, immediate, zeropage},
    compare::cmp,
    cpu::CPU,
};

pub fn cpx_immediate(cpu: &mut CPU) {
    let value = immediate(cpu);

    cmp(cpu, cpu.x, value);
    cpu.program_counter += 2;
}

pub fn cpx_zeropage(cpu: &mut CPU) {
    let value = cpu.memory[zeropage(cpu) as usize];

    cmp(cpu, cpu.x, value);
    cpu.program_counter += 2;
}

pub fn cpx_absolute(cpu: &mut CPU) {
    let value = cpu.memory[absolute(cpu) as usize];

    cmp(cpu, cpu.x, value);
    cpu.program_counter += 3;
}
