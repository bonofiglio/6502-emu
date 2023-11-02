use crate::{
    addressing::*,
    cpu::CPU,
    flags::{clear_c, clear_v, get_c, set_c, set_nz, set_v},
};

pub fn adc(cpu: &mut CPU, value: u8) {
    let value = value as u16;

    let first_positive = cpu.accumulator > 0 && cpu.accumulator < 128;
    let second_positive = value > 0 && value < 128;

    let both_positive = first_positive && second_positive;
    let both_negative = !first_positive && !second_positive;

    let carry = get_c(cpu);

    let result: u16 = cpu.accumulator as u16 + value + carry as u16;

    if result > 0xFF {
        set_c(cpu);
    } else {
        clear_c(cpu);
    }

    // get rid of the extra bits
    let result = result as u8;

    // If both were positive values and the result is positive, set the
    // overflow flag, do the same in case both were negative and the
    // result is negative
    if both_positive && result > 128 || both_negative && result < 128 {
        set_v(cpu);
    } else {
        clear_v(cpu);
    }

    cpu.accumulator = result;
    set_nz(cpu, cpu.accumulator);
}

pub fn adc_immediate(cpu: &mut CPU) {
    adc(cpu, immediate(cpu));
    cpu.program_counter += 2;
}

pub fn adc_zeropage(cpu: &mut CPU) {
    adc(cpu, cpu.memory[zeropage(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn adc_zeropage_x(cpu: &mut CPU) {
    adc(cpu, cpu.memory[zeropage_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn adc_absolute(cpu: &mut CPU) {
    adc(cpu, cpu.memory[absolute(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn adc_absolute_x(cpu: &mut CPU) {
    adc(cpu, cpu.memory[absolute_x(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn adc_absolute_y(cpu: &mut CPU) {
    adc(cpu, cpu.memory[absolute_y(cpu) as usize]);
    cpu.program_counter += 3;
}

pub fn adc_indirect_x(cpu: &mut CPU) {
    adc(cpu, cpu.memory[indirect_x(cpu) as usize]);
    cpu.program_counter += 2;
}

pub fn adc_indirect_y(cpu: &mut CPU) {
    adc(cpu, cpu.memory[indirect_y(cpu) as usize]);
    cpu.program_counter += 2;
}
