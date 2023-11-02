use crate::cpu::CPU;

pub const NP: u8 = 0b_1000_0000;
pub const VP: u8 = 0b_0100_0000;
pub const BP: u8 = 0b_0001_0000;
pub const DP: u8 = 0b_0000_1000;
pub const IP: u8 = 0b_0000_0100;
pub const ZP: u8 = 0b_0000_0010;
pub const CP: u8 = 0b_0000_0001;

pub fn get_n(cpu: &CPU) -> bool {
    cpu.status & NP == NP
}

pub fn set_n(cpu: &mut CPU) {
    cpu.status |= NP;
}

pub fn clear_n(cpu: &mut CPU) {
    cpu.status &= !NP;
}

pub fn get_z(cpu: &CPU) -> bool {
    cpu.status & ZP == ZP
}

pub fn clear_z(cpu: &mut CPU) {
    cpu.status &= !ZP;
}

pub fn set_z(cpu: &mut CPU) {
    cpu.status |= ZP
}

pub fn get_c(cpu: &CPU) -> bool {
    cpu.status & CP == CP
}

pub fn set_c(cpu: &mut CPU) {
    cpu.status |= CP;
}

pub fn clear_c(cpu: &mut CPU) {
    cpu.status &= !CP;
}

pub fn get_v(cpu: &CPU) -> bool {
    cpu.status & VP == VP
}

pub fn set_v(cpu: &mut CPU) {
    cpu.status |= VP;
}

pub fn clear_v(cpu: &mut CPU) {
    cpu.status &= !VP;
}

pub fn set_dp(cpu: &mut CPU) {
    cpu.status |= DP;
}

pub fn clear_dp(cpu: &mut CPU) {
    cpu.status &= !DP;
}

pub fn set_ip(cpu: &mut CPU) {
    cpu.status |= DP;
}

pub fn clear_ip(cpu: &mut CPU) {
    cpu.status &= !IP;
}

pub fn set_nz(cpu: &mut CPU, value: u8) {
    if value == 0 {
        set_z(cpu);
    } else {
        clear_z(cpu);
    };

    if value >= 0x80 {
        set_n(cpu);
    } else {
        clear_n(cpu);
    }
}
