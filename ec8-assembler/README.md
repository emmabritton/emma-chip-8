# EmmaChip8 Assembler

Assembler for EmmaChip8

## Usage

```
ec8-assembler [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  EC8 ASM file (*.eca)

Options:
  -o, --output [<FILE>]        Output file (defaults to input dir)
  -d, --desc [<FILE>]          Generate describe file
  -l, --level [<LevelFilter>]  Logging level [default: warn] [possible values: off, error, warn, info, debug, trace]
  -e, --ec8 [<CheckLevel>]     EC8 check level [default: warn] [possible values: off, warn, error]
  -h, --help                   Print help information
  -V, --version                Print version information
```

For example
`./ec8-assembler prog.eca`

## Language

Anything after semicolons is ignored, i.e. `ADD V0, V1 ;this is a comment`

To add data use `DAT <name> [<hex bytes>]`, i.e. `DAT example [3AFF0001]`

| Name                               | Mnemonic | Params        | Example         | Code   | Notes                                                        |
|------------------------------------|----------|---------------|-----------------|--------|--------------------------------------------------------------|
| Clear Display                      | `CLR`    |               | `CLR`           | `00E0` | Removes all sprites                                          |
| Return from subroutine             | `RET`    |               | `RET`           | `00EE` |                                                              |
| Jump to address                    | `JMP`    | Addr          | `JMP 1A1`       | `1nnn` |                                                              |
| Call subroutine                    | `CAL`    | Addr          | `CALL 1A1`      | `2nnn` |                                                              |
| Skip if reg == num                 | `SKE`    | Reg, Num      | `SKE V4, 45`    | `3xnn` |                                                              |
| Skip if reg != num                 | `SKN`    | Reg, Num      | `SKN VA, FF`    | `4xnn` |                                                              |
| Skip if reg == reg                 | `SKE`    | Reg, Reg      | `SKE VA, VF`    | `5xy0` |                                                              |
| Set reg to num                     | `SET`    | Reg, Num      | `SET V0, 88`    | `6xnn` |                                                              |
| Add num to reg                     | `ADD`    | Reg, Num      | `ADD V1, 4`     | `7xnn` |                                                              |
| Set reg to reg                     | `SET`    | Reg, Reg      | `V1, V2`        | `8xy0` |                                                              |
| Bitwise OR                         | `OR`     | Reg, Reg      | `V2, V3`        | `8xy1` | Vx &#124;= Vy                                                |
| Bitwise AND                        | `AND`    | Reg, Reg      | `V3, V4`        | `8xy2` | Vx &= Vy                                                     |
| Bitwise XOR                        | `XOR`    | Reg, Reg      | `V4, V5`        | `8xy3` | Vx ^= Vy                                                     |
| Add reg to reg                     | `ADD`    | Reg, Reg      | `V5, V6`        | `8xy4` | Vx += Vy                                                     |
| Sub reg from reg (l-r)             | `SUB`    | Reg, Reg      | `V6, V7`        | `8xy5` | Vx -= Vy                                                     |
| Shift Right                        | `SHR`    | Reg, Reg      | `V7, V8`        | `8xy6` | Sets VF to LSB of Vx. Shifts Vx right one                    |
| Sub reg from reg (r-l)             | `SBR`    | Reg, Reg      | `V8, V9`        | `8xy7` | Vx = Vy - Vx                                                 |
| Shift Left                         | `SHL`    | Reg, Reg      | `V9, VA`        | `8xyE` | Sets VF to MSB of Vx. Shifts Vx left one                     |
| Skip if reg != reg                 | `SKN`    | Reg, Reg      | `VA, VB`        | `9xy0` |                                                              |
| Set memory address                 | `STI`    | Addr          | `STI 4F2`       | `Annn` |                                                              |
| Jump to address+offset             | `JPO`    | Addr          | `JPO 10A`       | `Bnnn` | Jumps to addr + V0                                           |
| Random                             | `RND`    | Reg, Num      | `RND V1, FF`    | `Cxnn` | Set Vx to rand(0..=255) & nn                                 |
| Draw sprite                        | `DRW`    | Reg, Reg, Num | `DRW V0, V1, 5` | `Dxyn` | Draw sprite at Vx,Vy with n rows from I                      |
| Skip if key pressed                | `SKP`    | Reg           | `SKP V1`        | `Ex9E` |                                                              |
| Skip if key released               | `SKR`    | Reg           | `SKR V1`        | `ExA1` |                                                              |
| Set reg to delay                   | `RDT`    | Reg           | `RDT V1`        | `Fx07` |                                                              |
| Wait for key press                 | `KEY`    | Reg           | `KEY V0`        | `Fx0A` | Blocks execution                                             |
| Set delay timer                    | `SDT`    | Reg           | `SDT V9`        | `Fx15` |                                                              |
| Set sound timer                    | `SST`    | Reg           | `SST V7`        | `Fx18` |                                                              |
| Add to memory address              | `ADI`    | Reg           | `ADI V0`        | `Fx1E` | I += Vx                                                      |
| Set memory address to digit sprite | `CHR`    | Reg           | `CHR V1`        | `Fx29` |                                                              |
| Set memory address to ASCII sprite | `ASC`    | Reg           | `ASC V1`        | `Fx30` | Supports 0-9, a-z, A-Z, &#124;#!@$%^&*()_+-=[]{};'\:",./<>?~ |
| Write BCD to memory                | `BCD`    | Reg           | `BCD V5`        | `Fx33` | Hundreds at I, tens at I+1, ones at I+2                      |
| Store registers                    | `STR`    | Reg           | `STR VE`        | `Fx55` | Stores registers 0 - x in memory starting at I               |
| Load registers                     | `LDR`    | Reg           | `LDR VE`        | `Fx65` | Loads registers 0 - x from memory starting at I              |

Example Program
```
STI 0        ;Set I to 0
DRW V0,V1, 5 ;Draw '0' to 0,0
SET V3, F    ;Set V3 to xF
CHR V3       ;Set I to graphic for xF
SET V0, 5    ;Set V0 to x5
DRW V0,V1, 5 ;Draw 'F' to 0,5
```