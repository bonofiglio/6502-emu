use crate::cpu::CPU;

pub fn rts(cpu: &mut CPU) {
    cpu.program_counter = (cpu.stack_pop() as u16 | ((cpu.stack_pop() as u16) << 8)) + 1;
}
