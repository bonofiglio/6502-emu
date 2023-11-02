use crate::cpu::CPU;

pub fn rti(cpu: &mut CPU) {
    cpu.status = cpu.stack_pop() | 0b0011_0000;
    cpu.program_counter = cpu.stack_pop() as u16 | (cpu.stack_pop() as u16) << 8;
}
