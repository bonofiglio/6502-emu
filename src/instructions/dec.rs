use crate::{
    addressing::{absolute, zeropage, zeropage_x},
    cpu::CPU,
};

fn dec(cpu: &mut CPU, address: u16) {
    cpu.memory[address as usize] = cpu.memory[address as usize].wrapping_sub(1);
}

pub fn dec_zeropage(cpu: &mut CPU) {
    dec(cpu, zeropage(cpu) as u16);

    cpu.program_counter += 2;
}

pub fn dec_zeropage_x(cpu: &mut CPU) {
    dec(cpu, zeropage_x(cpu) as u16);

    cpu.program_counter += 2;
}

pub fn dec_absolute(cpu: &mut CPU) {
    dec(cpu, absolute(cpu));

    cpu.program_counter += 3;
}

pub fn dec_absolute_x(cpu: &mut CPU) {
    dec(cpu, absolute(cpu));

    cpu.program_counter += 3;
}
