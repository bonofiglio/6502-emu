use crate::{
    addressing::{
        absolute, absolute_x, absolute_y, immediate, indirect_x, indirect_y, zeropage, zeropage_x,
    },
    compare::cmp,
    cpu::CPU,
};

pub fn cmp_immediate(cpu: &mut CPU) {
    let value = immediate(cpu);

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 2;
}

pub fn cmp_zeropage(cpu: &mut CPU) {
    let value = cpu.memory[zeropage(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 2;
}

pub fn cmp_zeropage_x(cpu: &mut CPU) {
    let value = cpu.memory[zeropage_x(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 2;
}

pub fn cmp_absolute(cpu: &mut CPU) {
    let value = cpu.memory[absolute(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 3;
}

pub fn cmp_absolute_x(cpu: &mut CPU) {
    let value = cpu.memory[absolute_x(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 3;
}

pub fn cmp_absolute_y(cpu: &mut CPU) {
    let value = cpu.memory[absolute_y(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 3;
}

pub fn cmp_indirect_x(cpu: &mut CPU) {
    let value = cpu.memory[indirect_x(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 2;
}

pub fn cmp_indirect_y(cpu: &mut CPU) {
    let value = cpu.memory[indirect_y(cpu) as usize];

    cmp(cpu, cpu.accumulator, value);
    cpu.program_counter += 2;
}
