mod addressing;
mod compare;
mod cpu;
mod flags;
mod instructions;

use cpu::CPU;
use instructions::{
    adc::*, and::*, asl::*, bcc::*, bcs::*, beq::*, bit::*, bmi::*, bne::*, bpl::*, brk::*, bvc::*,
    bvs::*, clc::*, cld::*, cli::*, clv::*, cmp::*, cpx::*, cpy::*, dec::*, dex::*, dey::*, eor::*,
    inc::*, inx::*, iny::*, jmp::*, jsr::*, lda::*, ldx::*, ldy::*, lsr::*, nop::*, ora::*, pha::*,
    php::*, pla::*, plp::*, rol::*, ror::*, rti::*, rts::*, sbc::*, sec::*, sed::*, sei::*, sta::*,
    stx::*, sty::*, tax::*, tay::*, tsx::*, txa::*, txs::*, tya::*,
};

fn main() {
    let program_path = std::env::args().nth(1).unwrap();

    let program = std::fs::read(program_path).unwrap();

    let mut cpu = CPU {
        accumulator: 0x00,
        y: 0x00,
        x: 0x00,
        status: 0x36,
        program_counter: 0x0000,
        stack_pointer: 0xFF,
        memory: [0x00; 65536],
    };

    cpu.memory[..program.len()].copy_from_slice(&program);

    let mut command = String::new();

    println!("{}", cpu.to_string());

    execute_command(&cpu, &mut command);

    loop {
        let opcode = cpu.memory[cpu.program_counter as usize];

        match opcode {
            0x69 => adc_immediate(&mut cpu),
            0x65 => adc_zeropage(&mut cpu),
            0x75 => adc_zeropage_x(&mut cpu),
            0x6D => adc_absolute(&mut cpu),
            0x7D => adc_absolute_x(&mut cpu),
            0x79 => adc_absolute_y(&mut cpu),
            0x61 => adc_indirect_x(&mut cpu),
            0x71 => adc_indirect_y(&mut cpu),
            0x29 => and_immediate(&mut cpu),
            0x25 => and_zeropage(&mut cpu),
            0x35 => and_zeropage_x(&mut cpu),
            0x2D => and_absolute(&mut cpu),
            0x3D => and_absolute_x(&mut cpu),
            0x39 => and_absolute_y(&mut cpu),
            0x21 => and_indirect_x(&mut cpu),
            0x31 => and_indirect_y(&mut cpu),
            0x0A => asl_accumulator(&mut cpu),
            0x06 => asl_zeropage(&mut cpu),
            0x16 => asl_zeropage_x(&mut cpu),
            0x0E => asl_absolute(&mut cpu),
            0x1E => asl_absolute_x(&mut cpu),
            0x90 => bcc(&mut cpu),
            0xB0 => bcs(&mut cpu),
            0xF0 => beq(&mut cpu),
            0x24 => bit_zeropage(&mut cpu),
            0x2C => bit_absolute(&mut cpu),
            0x30 => bmi(&mut cpu),
            0xD0 => bne(&mut cpu),
            0x10 => bpl(&mut cpu),
            0x00 => brk(&mut cpu),
            0x50 => bvc(&mut cpu),
            0x70 => bvs(&mut cpu),
            0x18 => clc(&mut cpu),
            0xD8 => cld(&mut cpu),
            0x58 => cli(&mut cpu),
            0xB8 => clv(&mut cpu),
            0xC9 => cmp_immediate(&mut cpu),
            0xC5 => cmp_zeropage(&mut cpu),
            0xD5 => cmp_zeropage_x(&mut cpu),
            0xCD => cmp_absolute(&mut cpu),
            0xDD => cmp_absolute_x(&mut cpu),
            0xD9 => cmp_absolute_y(&mut cpu),
            0xC1 => cmp_indirect_x(&mut cpu),
            0xD1 => cmp_indirect_y(&mut cpu),
            0xE0 => cpx_immediate(&mut cpu),
            0xE4 => cpx_zeropage(&mut cpu),
            0xEC => cpx_absolute(&mut cpu),
            0xC0 => cpy_immediate(&mut cpu),
            0xC4 => cpy_zeropage(&mut cpu),
            0xCC => cpy_absolute(&mut cpu),
            0xC6 => dec_zeropage(&mut cpu),
            0xD6 => dec_zeropage_x(&mut cpu),
            0xCE => dec_absolute(&mut cpu),
            0xDE => dec_absolute_x(&mut cpu),
            0xCA => dex(&mut cpu),
            0x88 => dey(&mut cpu),
            0x49 => eor_immediate(&mut cpu),
            0x45 => eor_zeropage(&mut cpu),
            0x55 => eor_zeropage_x(&mut cpu),
            0x4D => eor_absolute(&mut cpu),
            0x5D => eor_absolute_x(&mut cpu),
            0x59 => eor_absolute_y(&mut cpu),
            0x41 => eor_indirect_x(&mut cpu),
            0x51 => eor_indirect_y(&mut cpu),
            0xE6 => inc_zeropage(&mut cpu),
            0xF6 => inc_zeropage_x(&mut cpu),
            0xEE => inc_absolute(&mut cpu),
            0xFE => inc_absolute_x(&mut cpu),
            0xE8 => inx(&mut cpu),
            0xC8 => iny(&mut cpu),
            0x4C => jmp_absolute(&mut cpu),
            0x6C => jmp_indirect(&mut cpu),
            0x20 => jsr(&mut cpu),
            0xA9 => lda_immediate(&mut cpu),
            0xA5 => lda_zeropage(&mut cpu),
            0xB5 => lda_zeropage_x(&mut cpu),
            0xAD => lda_absolute(&mut cpu),
            0xBD => lda_absolute_x(&mut cpu),
            0xB9 => lda_absolute_y(&mut cpu),
            0xA1 => lda_indirect_x(&mut cpu),
            0xB1 => lda_indirect_y(&mut cpu),
            0xA2 => ldx_immediate(&mut cpu),
            0xA6 => ldx_zeropage(&mut cpu),
            0xB6 => ldx_zeropage_y(&mut cpu),
            0xAE => ldx_absolute(&mut cpu),
            0xBE => ldx_absolute_y(&mut cpu),
            0xA0 => ldy_immediate(&mut cpu),
            0xA4 => ldy_zeropage(&mut cpu),
            0xB4 => ldy_zeropage_x(&mut cpu),
            0xAC => ldy_absolute(&mut cpu),
            0xBC => ldy_absolute_x(&mut cpu),
            0x4A => lsr_accumulator(&mut cpu),
            0x46 => lsr_zeropage(&mut cpu),
            0x56 => lsr_zeropage_x(&mut cpu),
            0x4E => lsr_absolute(&mut cpu),
            0x5E => lsr_absolute_x(&mut cpu),
            0xEA => nop(&mut cpu),
            0x09 => ora_immediate(&mut cpu),
            0x05 => ora_zeropage(&mut cpu),
            0x15 => ora_zeropage_x(&mut cpu),
            0x0D => ora_absolute(&mut cpu),
            0x1D => ora_absolute_x(&mut cpu),
            0x19 => ora_absolute_y(&mut cpu),
            0x01 => ora_indirect_x(&mut cpu),
            0x11 => ora_indirect_y(&mut cpu),
            0x48 => pha(&mut cpu),
            0x08 => php(&mut cpu),
            0x68 => pla(&mut cpu),
            0x28 => plp(&mut cpu),
            0x2A => rol_accumulator(&mut cpu),
            0x26 => rol_zeropage(&mut cpu),
            0x36 => rol_zeropage_x(&mut cpu),
            0x2E => rol_absolute(&mut cpu),
            0x3E => rol_absolute_x(&mut cpu),
            0x6A => ror_accumulator(&mut cpu),
            0x66 => ror_zeropage(&mut cpu),
            0x76 => ror_zeropage_x(&mut cpu),
            0x6E => ror_absolute(&mut cpu),
            0x7E => ror_absolute_x(&mut cpu),
            0x40 => rti(&mut cpu),
            0x60 => rts(&mut cpu),
            0xE9 => sbc_immediate(&mut cpu),
            0xE5 => sbc_zeropage(&mut cpu),
            0xF5 => sbc_zeropage_x(&mut cpu),
            0xED => sbc_absolute(&mut cpu),
            0xFD => sbc_absolute_x(&mut cpu),
            0xF9 => sbc_absolute_y(&mut cpu),
            0xE1 => sbc_indirect_x(&mut cpu),
            0xF1 => sbc_indirect_y(&mut cpu),
            0x38 => sec(&mut cpu),
            0xF8 => sed(&mut cpu),
            0x78 => sei(&mut cpu),
            0x85 => sta_zeropage(&mut cpu),
            0x95 => sta_zeropage_x(&mut cpu),
            0x8D => sta_absolute(&mut cpu),
            0x9D => sta_absolute_x(&mut cpu),
            0x99 => sta_absolute_y(&mut cpu),
            0x81 => sta_indirect_x(&mut cpu),
            0x91 => sta_indirect_y(&mut cpu),
            0x86 => stx_zeropage(&mut cpu),
            0x96 => stx_zeropage_y(&mut cpu),
            0x8E => stx_absolute(&mut cpu),
            0x84 => sty_zeropage(&mut cpu),
            0x94 => sty_zeropage_x(&mut cpu),
            0x8C => sty_absolute(&mut cpu),
            0xAA => tax(&mut cpu),
            0xA8 => tay(&mut cpu),
            0xBA => tsx(&mut cpu),
            0x8A => txa(&mut cpu),
            0x9A => txs(&mut cpu),
            0x98 => tya(&mut cpu),
            _ => unreachable!(),
        }

        println!("{}", cpu.to_string());

        execute_command(&cpu, &mut command);
    }
}

fn execute_command(cpu: &CPU, command: &mut String) {
    command.clear();
    println!("Next command (\x1b[93mq\x1b[0muit,\x1b[93md\x1b[0mump,<empty>): ");
    std::io::stdin().read_line(command).unwrap();

    match command.trim() {
        "q" | "quit" => std::process::exit(0),
        "d" | "dump" => {
            cpu.dump("output.log");
            println!("dumped to output.log");
            execute_command(cpu, command)
        }
        "" => (),
        _ => {
            println!("invalid command {}", command);
            execute_command(cpu, command)
        }
    }
}
