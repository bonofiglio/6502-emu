use crate::{
    addressing::{absolute, zeropage},
    cpu::CPU,
    flags::{clear_v, set_nz, set_v, VP},
};

fn bit(cpu: &mut CPU, value: u8) {
    let result = cpu.accumulator & value;

    set_nz(cpu, result);

    if result & VP == VP {
        set_v(cpu);
    } else {
        clear_v(cpu);
    }
}

pub fn bit_zeropage(cpu: &mut CPU) {
    bit(cpu, cpu.memory[zeropage(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn bit_absolute(cpu: &mut CPU) {
    bit(cpu, cpu.memory[absolute(cpu) as usize]);
    cpu.program_counter += 3;
}
