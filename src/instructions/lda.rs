use crate::addressing::*;
use crate::cpu::CPU;
use crate::flags::set_nz;

pub fn lda_immediate(cpu: &mut CPU) {
    cpu.accumulator = immediate(cpu);
    cpu.program_counter += 2;

    set_nz(cpu, cpu.accumulator);
}

pub fn lda_zeropage(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[zeropage(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.accumulator);
}

pub fn lda_zeropage_x(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[zeropage_x(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.accumulator);
}

pub fn lda_absolute(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[absolute(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.accumulator);
}

pub fn lda_absolute_x(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[absolute_x(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.accumulator);
}

pub fn lda_absolute_y(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[absolute_y(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.accumulator);
}

pub fn lda_indirect_x(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[indirect_x(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.accumulator);
}

pub fn lda_indirect_y(cpu: &mut CPU) {
    cpu.accumulator = cpu.memory[indirect_y(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.accumulator);
}
