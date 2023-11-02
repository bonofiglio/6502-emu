use crate::addressing::*;
use crate::cpu::CPU;
use crate::flags::set_nz;

pub fn ldx_immediate(cpu: &mut CPU) {
    cpu.x = immediate(cpu);
    cpu.program_counter += 2;
    set_nz(cpu, cpu.x);
}

pub fn ldx_zeropage(cpu: &mut CPU) {
    cpu.x = cpu.memory[zeropage(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.x);
}

pub fn ldx_zeropage_y(cpu: &mut CPU) {
    cpu.x = cpu.memory[zeropage_y(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.x);
}

pub fn ldx_absolute(cpu: &mut CPU) {
    cpu.x = cpu.memory[absolute(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.x);
}

pub fn ldx_absolute_y(cpu: &mut CPU) {
    cpu.x = cpu.memory[absolute_y(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.x);
}
