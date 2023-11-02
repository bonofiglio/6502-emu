use crate::cpu::CPU;

pub fn inx(cpu: &mut CPU) {
    cpu.x = cpu.x.wrapping_add(1);

    cpu.program_counter += 1;
}
