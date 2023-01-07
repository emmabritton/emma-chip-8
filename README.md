# EmmaChip8

Another Chip-8 emulator

## Usage

First run `build.sh` this will make the assembler and client.

Then run `run.sh <file` to build and run your program.

For example, if you had a file named `test.asm`:
```
build.sh
run.sh test.asm
```

then after making changes to `test.asm`, it's just `run.sh test.asm`.

The following structure will be made
```
/build
  /bin - contains the EC8 binaries
  /output - contains the assembled programs
```

The `run.sh` script also tells the assembler to create a description file (also in `output`) that describes what each line does.

If you have a binary file (often `.c8` or `.ch8`) you can use `exe.sh <file>`