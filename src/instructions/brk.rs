use crate::cpu::CPU;

pub fn brk(cpu: &mut CPU) {
    cpu.program_counter += 2;

    cpu.stack_push((cpu.program_counter >> 8) as u8);
    cpu.stack_push(cpu.program_counter as u8);
    cpu.stack_push(cpu.status);

    cpu.program_counter = cpu.memory[0xFFFE] as u16 | (cpu.memory[0xFFFF] as u16) << 8;
}
