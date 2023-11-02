use crate::{addressing::*, cpu::CPU};

use super::adc::adc;

fn sbc(cpu: &mut CPU, value: u8) {
    adc(cpu, 255u8.wrapping_sub(value));
}

pub fn sbc_immediate(cpu: &mut CPU) {
    sbc(cpu, immediate(cpu));
    cpu.program_counter += 2;
}

pub fn sbc_zeropage(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[zeropage(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn sbc_zeropage_x(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[zeropage_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn sbc_absolute(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[absolute(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn sbc_absolute_x(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[absolute_x(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn sbc_absolute_y(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[absolute_y(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn sbc_indirect_x(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[indirect_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn sbc_indirect_y(cpu: &mut CPU) {
    sbc(cpu, cpu.memory[indirect_y(cpu) as usize]);
    cpu.program_counter += 2;
}
