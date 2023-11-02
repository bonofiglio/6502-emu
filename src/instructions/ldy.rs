use crate::addressing::*;
use crate::cpu::CPU;
use crate::flags::set_nz;

pub fn ldy_immediate(cpu: &mut CPU) {
    cpu.y = immediate(cpu);
    cpu.program_counter += 2;
    set_nz(cpu, cpu.y);
}

pub fn ldy_zeropage(cpu: &mut CPU) {
    cpu.y = cpu.memory[zeropage(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.y);
}

pub fn ldy_zeropage_x(cpu: &mut CPU) {
    cpu.y = cpu.memory[zeropage_x(cpu) as usize];
    cpu.program_counter += 2;
    set_nz(cpu, cpu.y);
}

pub fn ldy_absolute(cpu: &mut CPU) {
    cpu.y = cpu.memory[absolute(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.y);
}

pub fn ldy_absolute_x(cpu: &mut CPU) {
    cpu.y = cpu.memory[absolute_x(cpu) as usize];
    cpu.program_counter += 3;
    set_nz(cpu, cpu.y);
}
