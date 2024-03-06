This is a compiler for the stack-based programming language [CLAC](https://www.cs.cmu.edu/afs/cs.cmu.edu/academic/class/15122-s14/www/prog4.pdf). It generates x86 assembly code for Linux systems.

## Requirements
- Rust
- NASM
- gcc


## Usage (automatic compilation is unfinished)

1. Write your CLAC source code

2. Run the CLAC compiler using the following command:
```
cargo run main.clac
```
This will generate an assembly file named `output.asm`.

3. Assemble the generated assembly file using NASM:
```
nasm -f elf64 output.asm
```

This will produce an object file named `output.o`.

4. Link the object file using the GNU linker:
```
gcc -no-pie -o output output.o
```

This will create an executable file named `output`.

5. Run the compiled program:
```
./output
```

