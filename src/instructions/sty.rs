use crate::{addressing::*, cpu::CPU};

pub fn sty_zeropage(cpu: &mut CPU) {
    cpu.memory[zeropage(cpu) as usize] = cpu.y;
    cpu.program_counter += 2;
}

pub fn sty_zeropage_x(cpu: &mut CPU) {
    cpu.memory[zeropage_x(cpu) as usize] = cpu.y;
    cpu.program_counter += 2;
}

pub fn sty_absolute(cpu: &mut CPU) {
    cpu.memory[absolute(cpu) as usize] = cpu.y;
    cpu.program_counter += 3;
}
