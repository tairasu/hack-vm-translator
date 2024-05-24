# Hack VM Translator

A Hack Virtual Machine (VM) translator written in Rust. It translates VM code into Hack assembly code, which can then be assembled and run on the Hack computer as defined in the Nand2Tetris course.

## Features

- Translates VM commands into Hack assembly code.
- Supports arithmetic operations, memory access, branching, and function calls.
- Efficiently handles static and dynamic content in the generated assembly code.
- Command-line interface for specifying the input VM file.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (make sure you have the latest version installed)

## Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/tairasu/hack-vm-translator.git
   cd hack-vm-translator
   ```

2. Build the project using Cargo:

   ```sh
   cargo build --release
   ```

## Usage

To use the translator, run the binary (compiled in /target/release) with the path to your `.vm` file as an argument:

```sh
./hack-vm path/to/your/file.vm
```

For example:

```sh
./hack-vm 7/StackArithmetic/StackTest/StackTest.vm
```

This will generate a `.asm` file with the same name in the same directory.

## Example

Given a VM file `StackTest.vm` with the following content:

```vm
push constant 7
push constant 8
add
```

Running the translator:

```sh
./hack-vm 7/StackArithmetic/StackTest/StackTest.vm
```

Will produce a `StackTest.asm` file with the translated Hack assembly code:

```asm
@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@8
D=A
@SP
A=M
M=D
@SP
M=M+1
@SP
M=M-1
A=M
D=M
@SP
M=M-1
A=M
M=D+M
@SP
M=M+1
```

## Project Structure

- `main.rs`: The entry point of the application. Handles command-line arguments and file operations.
- `lib.rs`: Contains the core logic for parsing and translating VM commands.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project is inspired by the [Nand2Tetris](https://www.nand2tetris.org/) course.