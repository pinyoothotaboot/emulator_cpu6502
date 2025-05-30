# 6502 CPU Emulator in Rust

[![Rust](https://github.com/pinyoothotaboot/emulator_cpu6502/actions/workflows/rust.yml/badge.svg)](https://github.com/pinyoothotaboot/emulator_cpu6502/actions/workflows/rust.yml)

A cycle-accurate MOS 6502 CPU emulator written in Rust. This project aims to provide a clean, well-tested implementation of the 6502 microprocessor that can be used as a building block for emulating classic 8-bit computers and game consoles.

## Features

- 🚀 Cycle-accurate 6502 CPU emulation
- 🧪 Comprehensive test suite
- 🛠️ Modular design with clean interfaces
- 📚 Complete instruction set implementation
- 🧠 Support for all addressing modes
- 🔄 Interrupt handling (NMI, IRQ, BRK, RESET)

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (Rust's package manager)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/pinyoothotaboot/emulator_cpu6502.git
   cd emulator_cpu6502
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Running Tests

To run the test suite:

```bash
cargo test
```

## Usage

Here's a basic example of how to use the emulator:

```rust
use emulator_cpu6502::cpu::model::CPU;
use emulator_cpu6502::bus::model::Bus;
use emulator_cpu6502::interfaces::cpu::ICPU;

fn main() {
    // Create a new CPU and bus
    let mut cpu = CPU::new();
    let mut bus = Bus::new();
    
    // Reset the CPU
    cpu.reset();
    
    // Load a simple program into memory
    // LDA #$42
    // STA $0200
    let program = [0xA9, 0x42, 0x8D, 0x00, 0x02, 0x00];
    for (i, &byte) in program.iter().enumerate() {
        bus.write(&(0x8000 + i as u16), byte);
    }
    
    // Run the CPU
    // In a real emulator, you would run this in a loop
    // while !cpu.halted() {
    //     cpu.step(&mut bus);
    // }
}
```

## Project Structure

```
emulator_cpu6502/
├── src/
│   ├── bus/           # Memory bus implementation
│   ├── cpu/           # CPU core implementation
│   │   ├── model.rs   # CPU data structures
│   │   ├── operations/# Instruction implementations
│   │   └── ...
│   ├── interfaces/    # Trait definitions
│   └── main.rs        # Example usage
├── tests/             # Integration tests
└── Cargo.toml         # Project configuration
```

## Resources

- [6502 Reference](https://www.masswerk.at/6502/6502_instruction_set.html) - Complete 6502 instruction set reference
- [Easy 6502](https://skilldrick.github.io/easy6502/) - Interactive 6502 assembler/emulator
- [6502.org](http://www.6502.org/) - Comprehensive 6502 resources

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.