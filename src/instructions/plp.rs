use crate::cpu::CPU;

pub fn plp(cpu: &mut CPU) {
    cpu.status = cpu.stack_pop();

    cpu.program_counter += 1;
}
