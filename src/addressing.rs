use crate::cpu::CPU;

pub fn immediate(cpu: &CPU) -> u8 {
    cpu.memory[(cpu.program_counter.wrapping_add(1)) as usize]
}

pub fn zeropage(cpu: &CPU) -> u8 {
    immediate(cpu)
}

pub fn zeropage_x(cpu: &CPU) -> u8 {
    zeropage(cpu).wrapping_add(cpu.x)
}

pub fn zeropage_y(cpu: &CPU) -> u8 {
    zeropage(cpu).wrapping_add(cpu.x)
}

pub fn relative(cpu: &CPU) -> u8 {
    immediate(cpu)
}

pub fn absolute(cpu: &CPU) -> u16 {
    let byte_1 = cpu.memory[(cpu.program_counter.wrapping_add(1)) as usize];
    let byte_2 = cpu.memory[(cpu.program_counter.wrapping_add(2)) as usize];
    ((byte_2 as u16) << 8) | byte_1 as u16
}

pub fn absolute_x(cpu: &CPU) -> u16 {
    let byte_1 = cpu.memory[(cpu.program_counter.wrapping_add(1)) as usize];
    let byte_2 = cpu.memory[(cpu.program_counter.wrapping_add(2)) as usize];
    ((byte_2 as u16) << 8 | byte_1 as u16).wrapping_add(cpu.x as u16)
}

pub fn absolute_y(cpu: &CPU) -> u16 {
    let byte_1 = cpu.memory[(cpu.program_counter.wrapping_add(1)) as usize];
    let byte_2 = cpu.memory[(cpu.program_counter.wrapping_add(2)) as usize];
    ((byte_2 as u16) << 8 | byte_1 as u16).wrapping_add(cpu.y as u16)
}

pub fn indirect(cpu: &CPU) -> u16 {
    let absolute_address = absolute(cpu);

    let byte_1 = cpu.memory[absolute_address as usize];
    let byte_2 = cpu.memory[(absolute_address + 1) as usize];

    ((byte_2 as u16) << 8) | byte_1 as u16
}

pub fn indirect_x(cpu: &CPU) -> u16 {
    let immediate = cpu.memory[(cpu.program_counter.wrapping_add(1)) as usize];
    let target_address = immediate.wrapping_add(cpu.x);

    let byte_1 = cpu.memory[target_address as usize];
    let byte_2 = cpu.memory[(target_address.wrapping_add(1)) as usize];

    (byte_2 as u16) << 8 | byte_1 as u16
}

pub fn indirect_y(cpu: &CPU) -> u16 {
    let immediate = cpu.memory[(cpu.program_counter.wrapping_add(1)) as usize];
    let byte_1 = cpu.memory[immediate as usize];
    let byte_2 = cpu.memory[(immediate.wrapping_add(1)) as usize];

    ((byte_2 as u16) << 8 | byte_1 as u16).wrapping_add(cpu.y as u16)
}
