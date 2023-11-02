use crate::cpu::CPU;

pub fn pha(cpu: &mut CPU) {
    cpu.stack_push(cpu.accumulator);
    cpu.program_counter += 1;
}
