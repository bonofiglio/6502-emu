use crate::cpu::CPU;

pub fn pla(cpu: &mut CPU) {
    cpu.accumulator = cpu.stack_pop();

    cpu.program_counter += 1;
}
