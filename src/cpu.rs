use std::{fmt::Display, fs::File, io::Write};

pub type Memory = [u8; 65536];

pub struct CPU {
    pub accumulator: u8,
    pub y: u8,
    pub x: u8,
    // negative, overflow, noop, break, decimal, interrupt, zero, carry
    pub status: u8,
    pub program_counter: u16,
    pub stack_pointer: u8,

    pub memory: Memory,
}

impl CPU {
    pub fn dump(&self, filename: &str) {
        let mut s = String::with_capacity(self.memory.len() * 3 + ((self.memory.len() / 8) * 6));

        for (i, chunk) in self.memory.chunks(8).enumerate() {
            s.push_str(
                format!(
                    "{:04X}: {}\n",
                    i * 8,
                    chunk
                        .iter()
                        .map(|b| format!("{:02X}", b))
                        .collect::<Vec<String>>()
                        .join(" ")
                )
                .as_str(),
            );
        }

        let mut file = File::create(filename).expect("Failed to create file");
        file.write_all(
            format!(
                "PC: {:04X}\nAC: {:02X}\nXR: {:02X}\nYR: {:02X}\nSR: {:02X}\nSP: {:02X}\n\n\n{}",
                self.program_counter,
                self.accumulator,
                self.x,
                self.y,
                self.status,
                self.stack_pointer,
                s
            )
            .as_bytes(),
        )
        .expect("Failed to write to file");
    }

    pub fn stack_push(&mut self, value: u8) {
        self.memory[(0x0100 + self.stack_pointer as u16) as usize] = value;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    pub fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        self.memory[(0x0100 + self.stack_pointer as u16) as usize]
    }
}

impl Display for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            " PC   AC XR YR SP NV-BDIZC\n{:04X}  {:02X} {:02X} {:02X} {:02X} {:08b}",
            self.program_counter, self.accumulator, self.x, self.y, self.stack_pointer, self.status
        )
    }
}
