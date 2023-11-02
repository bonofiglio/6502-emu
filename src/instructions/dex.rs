use crate::cpu::CPU;

pub fn dex(cpu: &mut CPU) {
    cpu.x = cpu.x.wrapping_sub(1);

    cpu.program_counter += 1;
}
