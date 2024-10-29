## bf-interpreter

This is a simple Brainfuck interpreter written in Rust. It takes a file path as command line input, parses the Brainfuck code within the file, and executes the program.

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-username/bf-interpreter.git
2. Navigate to the project directory:
   ```bash
   cd bf-interpreter
3. Build the project:
   ```bash
   cargo build --release

### Usage
To run the interpreter, use the following command:
    
    cargo run --release <path/to/brainfuck_file.bf>

### Features
-File input: Takes a Brainfuck program file as command line input.
-Basic Brainfuck instruction support:
  - \>: Move the pointer to the right.
  - <: Move the pointer to the left.
  - +: Increment the value at the current pointer.
  - -: Decrement the value at the current pointer.
  - .: Output the value at the current pointer as an ASCII character.
  - ,: Input a character and store its ASCII value at the current pointer.
  - \[: Start a loop. The loop continues as long as the value at the current pointer is not zero.
  - ]: End a loop.
### Contributing
Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or improvements.
