# EmmaChip8

Another Chip-8 emulator

It has one extra instruction (Fx31, ASC or ascii) that sets I to the graphics for an ASCII character like Fx29

## Usage

First run `build.sh` this will make the assembler and client.

Then run `run.sh <file` to build and run your program.

For example, if you had a file named `test.asm`:
```
./scripts/build.sh
./scripts/run-asm.sh test.asm
```

then after making changes to `test.asm`, it's just `run.sh test.asm`.

The following structure will be made
```
/build
  /bin - contains the EC8 binaries
  /output - contains the assembled programs
```

The `run-asm.sh` script also tells the assembler to create a description file (also in `output`) that describes what each line does.

If you have a binary file (often `.c8` or `.ch8`) you can use `exe.sh <file>`.

For `run-asm.sh`, `run-ll.sh` and `exe.sh`, you can pass `log` as the first param and the file in the second, e.g. `exe.sh log test.c8`