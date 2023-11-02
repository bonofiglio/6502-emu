use crate::{addressing::*, cpu::CPU};

pub fn stx_zeropage(cpu: &mut CPU) {
    cpu.memory[zeropage(cpu) as usize] = cpu.x;
    cpu.program_counter += 2;
}

pub fn stx_zeropage_y(cpu: &mut CPU) {
    cpu.memory[zeropage_y(cpu) as usize] = cpu.x;
    cpu.program_counter += 2;
}

pub fn stx_absolute(cpu: &mut CPU) {
    cpu.memory[absolute(cpu) as usize] = cpu.x;
    cpu.program_counter += 3;
}
