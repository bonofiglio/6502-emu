use crate::{addressing::*, cpu::CPU, flags::set_nz};

fn and(cpu: &mut CPU, value: u8) {
    cpu.accumulator &= value;
    set_nz(cpu, cpu.accumulator);
}

pub fn and_immediate(cpu: &mut CPU) {
    and(cpu, immediate(cpu));
    cpu.program_counter += 2;
}

pub fn and_zeropage(cpu: &mut CPU) {
    and(cpu, cpu.memory[zeropage(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn and_zeropage_x(cpu: &mut CPU) {
    and(cpu, cpu.memory[zeropage_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn and_absolute(cpu: &mut CPU) {
    and(cpu, cpu.memory[absolute(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn and_absolute_x(cpu: &mut CPU) {
    and(cpu, cpu.memory[absolute_x(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn and_absolute_y(cpu: &mut CPU) {
    and(cpu, cpu.memory[absolute_y(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn and_indirect_x(cpu: &mut CPU) {
    and(cpu, cpu.memory[indirect_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn and_indirect_y(cpu: &mut CPU) {
    and(cpu, cpu.memory[indirect_y(cpu) as usize]);
    cpu.program_counter += 2;
}
