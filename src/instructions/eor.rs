use crate::{addressing::*, cpu::CPU, flags::set_nz};

fn eor(cpu: &mut CPU, value: u8) {
    cpu.accumulator ^= value;
    set_nz(cpu, cpu.accumulator);
}

pub fn eor_immediate(cpu: &mut CPU) {
    eor(cpu, immediate(cpu));
    cpu.program_counter += 2;
}

pub fn eor_zeropage(cpu: &mut CPU) {
    eor(cpu, cpu.memory[zeropage(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn eor_zeropage_x(cpu: &mut CPU) {
    eor(cpu, cpu.memory[zeropage_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn eor_absolute(cpu: &mut CPU) {
    eor(cpu, cpu.memory[absolute(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn eor_absolute_x(cpu: &mut CPU) {
    eor(cpu, cpu.memory[absolute_x(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn eor_absolute_y(cpu: &mut CPU) {
    eor(cpu, cpu.memory[absolute_y(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn eor_indirect_x(cpu: &mut CPU) {
    eor(cpu, cpu.memory[indirect_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn eor_indirect_y(cpu: &mut CPU) {
    eor(cpu, cpu.memory[indirect_y(cpu) as usize]);
    cpu.program_counter += 2;
}
