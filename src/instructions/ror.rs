use crate::{
    addressing::*,
    cpu::CPU,
    flags::{get_c, set_c, set_nz},
};

fn ror(cpu: &mut CPU, value: &mut u8) {
    let result = *value >> 1 | ((get_c(cpu) as u8) << 7);

    if *value & 1 == 1 {
        set_c(cpu);
    }

    *value = result;
    set_nz(cpu, *value);
}

pub fn ror_accumulator(cpu: *mut CPU) {
    let accumulator = unsafe { &mut (&mut *cpu).accumulator };
    let cpu = unsafe { &mut *cpu };

    ror(cpu, accumulator);
    cpu.program_counter += 2;
}

pub fn ror_zeropage(cpu: *mut CPU) {
    let zeropage = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(zeropage(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    ror(cpu, zeropage);
    cpu.program_counter += 2;
}

pub fn ror_zeropage_x(cpu: *mut CPU) {
    let zeropage_x = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(zeropage_x(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    ror(cpu, zeropage_x);
    cpu.program_counter += 2;
}

pub fn ror_absolute(cpu: *mut CPU) {
    let absolute = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(absolute(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    ror(cpu, absolute);
    cpu.program_counter += 2;
}

pub fn ror_absolute_x(cpu: *mut CPU) {
    let absolute_x = unsafe {
        let cpu_mut = &mut *cpu;
        let cpu = &*cpu;

        cpu_mut.memory.get_mut(absolute_x(cpu) as usize).unwrap()
    };
    let cpu = unsafe { &mut *cpu };

    ror(cpu, absolute_x);
    cpu.program_counter += 2;
}
