use crate::cpu::CPU;

pub fn php(cpu: &mut CPU) {
    cpu.stack_push(cpu.status);
    cpu.program_counter += 1;
}
