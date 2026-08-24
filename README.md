# CHIP-8 Emulator

A CHIP-8 emulator written in Rust from scratch.

CHIP-8 is an interpreted programming language from the late 1970s, originally designed for the COSMAC VIP and Telmac 1800 microcomputers. This emulator implements the full CHIP-8 instruction set and can run classic games like Pong, Tic-Tac-Toe, and Space Invaders.

![Demo](media/demo.gif)

## Features

- All 34 CHIP-8 instructions implemented
- 64×32 pixel display rendered with `minifb`
- Sound support via `rodio` (buzzer tone when sound timer > 0)
- Keyboard input mapped to CHIP-8's hex keypad
- 60 FPS display refresh with ~600 CPU instructions/second

## Build

Requires [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo build --release
```

The binary will be at `target/release/chip-8-emulator.exe` (Windows).

## Usage

```bash
chip-8-emulator.exe <path-to-rom>
```

Press **Escape** to exit.

## Keyboard Mapping

The CHIP-8 uses a 16-key hex keypad (0–F). Keys are mapped as follows:

| CHIP-8 Key | Keyboard Key |
|:---:|:---:|
| 0 | 0 |
| 1 | 1 |
| 2 | 2 |
| 3 | 3 |
| 4 | 4 |
| 5 | 5 |
| 6 | 6 |
| 7 | 7 |
| 8 | 8 |
| 9 | 9 |
| A | A |
| B | B |
| C | C |
| D | D |
| E | E |
| F | F |


## Project Structure

```
src/
├── main.rs        # Entry point — parses CLI args, launches emulator
├── emulator.rs    # Window, input, timing, sound — the main loop
├── chip8.rs       # CPU state (registers, RAM, display) and tick cycle
└── opcodes.rs     # All 34 instruction implementations
```

## Technical Details

- **CPU**: 16 general-purpose 8-bit registers (V0–VF), 16-bit index register, 16-bit program counter, 16-level stack
- **Memory**: 4096 bytes of RAM; programs loaded at address 0x200
- **Display**: 64×32 monochrome pixels, drawn via XOR sprite rendering
- **Timers**: Delay and sound timers decrement at 60 Hz
- **Timing**: 10 instructions (fetch-decode-execute) per frame × 60 FPS ≈ 600 instructions/second

## References & Acknowledgements

- [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) by Tobias V. I. Langhoff — technical reference
- [CHIP-8 Test Suite](https://github.com/Timendus/chip8-test-suite) by Timendus — opcode test ROMs
- [chip8-test-rom](https://github.com/corax89/chip8-test-rom) by corax89 — Corax+ opcode test ROM
- [CHIP-8 ROM's](https://github.com/badlogic/chip8/tree/master/roms) by badlogic — ROM pool

## License

[MIT](LICENSE)

