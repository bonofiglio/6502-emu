use crate::cpu::CPU;

pub fn dey(cpu: &mut CPU) {
    cpu.y = cpu.y.wrapping_sub(1);

    cpu.program_counter += 1;
}
