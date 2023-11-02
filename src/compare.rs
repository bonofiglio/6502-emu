use crate::{
    cpu::CPU,
    flags::{clear_c, clear_n, clear_z, set_c, set_n, set_z},
};

pub fn cmp(cpu: &mut CPU, register: u8, value: u8) {
    if register >= value {
        set_c(cpu);
    } else {
        clear_c(cpu);
    }

    if register == value {
        set_z(cpu);
    } else {
        clear_z(cpu);
    }

    if value >= 0x80 {
        set_n(cpu);
    } else {
        clear_n(cpu);
    }

    cpu.program_counter += 2;
}
