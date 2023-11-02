use crate::cpu::CPU;

pub fn iny(cpu: &mut CPU) {
    cpu.y = cpu.x.wrapping_add(1);

    cpu.program_counter += 1;
}
