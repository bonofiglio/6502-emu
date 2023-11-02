use crate::{addressing::*, cpu::CPU, flags::set_nz};

fn ora(cpu: &mut CPU, value: u8) {
    cpu.accumulator |= value;
    set_nz(cpu, cpu.accumulator);
}

pub fn ora_immediate(cpu: &mut CPU) {
    ora(cpu, immediate(cpu));
    cpu.program_counter += 2;
}

pub fn ora_zeropage(cpu: &mut CPU) {
    ora(cpu, cpu.memory[zeropage(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn ora_zeropage_x(cpu: &mut CPU) {
    ora(cpu, cpu.memory[zeropage_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn ora_absolute(cpu: &mut CPU) {
    ora(cpu, cpu.memory[absolute(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn ora_absolute_x(cpu: &mut CPU) {
    ora(cpu, cpu.memory[absolute_x(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn ora_absolute_y(cpu: &mut CPU) {
    ora(cpu, cpu.memory[absolute_y(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn ora_indirect_x(cpu: &mut CPU) {
    ora(cpu, cpu.memory[indirect_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn ora_indirect_y(cpu: &mut CPU) {
    ora(cpu, cpu.memory[indirect_y(cpu) as usize]);
    cpu.program_counter += 2;
}
