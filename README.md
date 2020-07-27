# Brainfrick Rust

A [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) compiler written in Rust, using LLVM

## Building the code

The code uses [cargo](https://doc.rust-lang.org/cargo/) to build and run the code, and uses the 
[inkwell](https://github.com/TheDan64/inkwell) crate to access the LLVM API.

Inkwell requires you to have LLVM 10 installed on your system to compile this from source. For instructions on how to install LLVM, look here: https://clang.llvm.org/get_started.html

## Compiling and Running Code

`brainfrick-rust` requires two requires a source file and a `-o OUTPUT_PATH` so the compiler knows where to put the compiled code.

`brainfrick-rust` produces object files, and the compiled code must be linked manually. The easiest way to do this is to run `gcc OUTPUT_PATH -o EXECUTABLE_PATH`

## Examples

The `samples` directory contains a few examples from the Wikipedia page and 
can be run via `cargo run FILENAME -o OUTPUT_PATH`, then linking it with `gcc` as described above

### Addition

```
cargo run samples/addition.bf -o tests/addition.o
gcc tests/addition.o -o tests/addition.exe
./tests/addition.exe 
7
```

### Hello World

```
cargo run samples/addition.bf -o tests/addition.o
gcc tests/hello_world.o -o tests/hello_world.exe
./tests/hello_world.exe
Hello World!
```

### ROT13 Cipher

```
cargo run samples/ROT13.bf -o tests/ROT13.o
gcc tests/ROT13.o -o tests/ROT13.exe
./tests/ROT13.exe
asdfasdf
nfqsnfqs
zxcv
mkpi
^C⏎ 
```