use crate::{
    addressing::{absolute, zeropage, zeropage_x},
    cpu::CPU,
};

fn inc(cpu: &mut CPU, address: u16) {
    cpu.memory[address as usize] = cpu.memory[address as usize].wrapping_add(1);
}

pub fn inc_zeropage(cpu: &mut CPU) {
    inc(cpu, zeropage(cpu) as u16);

    cpu.program_counter += 2;
}

pub fn inc_zeropage_x(cpu: &mut CPU) {
    inc(cpu, zeropage_x(cpu) as u16);

    cpu.program_counter += 2;
}

pub fn inc_absolute(cpu: &mut CPU) {
    inc(cpu, absolute(cpu));

    cpu.program_counter += 3;
}

pub fn inc_absolute_x(cpu: &mut CPU) {
    inc(cpu, absolute(cpu));

    cpu.program_counter += 3;
}
