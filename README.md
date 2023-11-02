# Rust 6502 Emulator

This emulator allows you to simulate the execution of 6502 CPU instructions and provides a few controls to interact with it. You can use it to run and debug 6502 assembly code.

## Building

To build the emulator, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from [Rust's official website](https://www.rust-lang.org/).

To compile and run the emulator, follow these steps:

1. Clone this repository:
   ```
   git clone https://github.com/your-username/6502-emulator-rust.git
   ```

2. Change your working directory to the project folder:
   ```
   cd 6502-emulator-rust
   ```

3. Compile and run the emulator:
   ```
   cargo build --release
   ./target/release/emu_6502 ./examples/multiply_by_10.bin
   ```

## Usage

Once the emulator is running, you can interact with it using the following controls:

- **Quit (q)**: Typing "quit" or "q" will quit the execution and close the emulator.

- **Dump (d)**: Typing "dump" or "d" will save the current content of the CPU registers and memory to a file named "output.log." This can be useful for debugging or analyzing the program's state.

- **Empty**: If you press the Enter key without entering any command, the emulator will advance the execution by one instruction. This allows you to step through the program one instruction at a time.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.

## Contributing

Feel free to contribute to this project by opening issues, providing feedback, or submitting pull requests.
