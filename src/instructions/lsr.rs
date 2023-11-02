use crate::{
    addressing::*,
    cpu::CPU,
    flags::{set_c, set_nz},
};

fn lsr(cpu: &mut CPU, value: &mut u8) {
    let result: u16 = (*value as u16) >> 1;

    if *value & 1 == 1 {
        set_c(cpu);
    }

    *value = result as u8;
    set_nz(cpu, *value);
}

pub fn lsr_accumulator(cpu: *mut CPU) {
    let accumulator = unsafe { &mut (&mut *cpu).accumulator };
    let cpu = unsafe { &mut *cpu };

    lsr(cpu, accumulator);
    cpu.program_counter += 1;
}

pub fn lsr_zeropage(cpu: *mut CPU) {
    let zeropage = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(zeropage(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    lsr(cpu, zeropage);
    cpu.program_counter += 2;
}

pub fn lsr_zeropage_x(cpu: *mut CPU) {
    let zeropage_x = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(zeropage_x(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    lsr(cpu, zeropage_x);
    cpu.program_counter += 2;
}

pub fn lsr_absolute(cpu: *mut CPU) {
    let absolute = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(absolute(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    lsr(cpu, absolute);
    cpu.program_counter += 3;
}

pub fn lsr_absolute_x(cpu: *mut CPU) {
    let absolute_x = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(absolute_x(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    lsr(cpu, absolute_x);
    cpu.program_counter += 3;
}
