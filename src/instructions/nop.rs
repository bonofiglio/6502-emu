use crate::cpu::CPU;

pub fn nop(cpu: &mut CPU) {
    cpu.program_counter += 1;
}
