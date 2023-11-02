use crate::{addressing::*, cpu::CPU};

pub fn sta_zeropage(cpu: &mut CPU) {
    cpu.memory[zeropage(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 2;
}

pub fn sta_zeropage_x(cpu: &mut CPU) {
    cpu.memory[zeropage_x(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 2;
}

pub fn sta_absolute(cpu: &mut CPU) {
    cpu.memory[absolute(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 3;
}

pub fn sta_absolute_x(cpu: &mut CPU) {
    cpu.memory[absolute_x(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 3;
}

pub fn sta_absolute_y(cpu: &mut CPU) {
    cpu.memory[absolute_y(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 3;
}

pub fn sta_indirect_x(cpu: &mut CPU) {
    cpu.memory[indirect_x(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 2;
}

pub fn sta_indirect_y(cpu: &mut CPU) {
    cpu.memory[indirect_x(cpu) as usize] = cpu.accumulator;
    cpu.program_counter += 2;
}
