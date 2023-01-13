# EmmaChip8 LL Compiler

Compiler for EmmaChip8 Low Level Language

## Usage

```
ec8-ll-compiler [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  EC8 code file (*.ecc)

Options:
  -o, --output [<FILE>]          Output file (defaults to input dir)
  -l, --level [<LevelFilter>]    Logging level [default: warn] [possible values: off, error, warn, info, debug, trace]
  -e, --ec8 [<CheckLevel>]       EC8 check level [default: warn] [possible values: off, warn, error]
  -w, --warnings [<CheckLevel>]  Lint/warnings check level [default: warn] [possible values: off, warn, error]
  -h, --help                     Print help information
  -V, --version                  Print version information
```

## Language

This is a low level language, that makes the assembly nicer to write by expanding mnemonics into keywords, adding labels.

- `r` Data register
- `lbl` Label
- `n` Number between 0 and 15
- `nn` Number between -128 and 255
- `nnn` Number between 0 and 4095

### Literals

- `'a'` ASCII letter
- `3` Decimal number
- `x3` Hexadecimal number
- `b1011` Binary number

### Constants

- `PROG` `0x200` Start of program in memory
- `G_DIGIT` `0x000` Start of digits graphics in memory
- `G_ALPHA` `0x032` Start of letters graphics in memory
- `G_SYM` `0x0B4` Start of symbols graphics in memory

### Variables

- `Vx` Data register 0 to F
- `I` Memory register
- `delay` Delay register
- `sound` Sound register

### Syntax

#### General

- `<name>:` Defines a label
- `data <name> <bytes>` Defines a data block
- `alias <name> <value>` Creates an alias

#### Statements
- `Vx = [Vy | nn | delay]` Set lhs to rhs
- `delay = Vx` 
- `Vx += [Vy | nn]` Set x to x + y
- `Vx -= Vy` Set x to x - y
- `Vx = Vy - Vx` Set x to y - x
- `Vx -= Vy` Set x to x | y
- `Vx &= Vy` Set x to x & y
- `Vx ^= Vy` Set x to x ^ y
- `shr(Vx)`
- `shl(Vx)`
- `I = lbl`
- `I += Vx`
- `rand(Vx, nn)`
- `draw(Vx, Vy, n)` Draw sprite at x, y with n rows from I
- `digit(Vx)` Set I to hex digit in x
- `ascii(Vy)` Set I to ascii letter in x
- `goto(lbl)` Jump to lbl
- `call(lbl)` Call lbl as subroutine
- `return` Return from subroutine
- `clear` Clear display
- `goto(lbl, Vx)` 
- `reg_store(Vx)`
- `reg_load(Vx)`
- `bcd(Vx)`
- `wait_for_key(Vx)`
- `break`

#### Conditionals

Can be preceded by `!` to negate

- `eq(Vx,Vy | nn)`
- `pressed(Vx)`

#### Flow

- `if <conditional> <statement>`
- `loop`
- `again`

#### Macros

All of these builtin ones clobber `I`
- `draw_digit!(Vx, Vy, Vd)` Draw digit Vd at Vx, Vy
- `draw_ascii!(Vx, Vy, Vc)` Draw ascii Vc at Vx, Vy
- `read_data!(lbl, Vx)` Set V0 to byte at lbl + Vx

#### Defining a Macro

```
macro(example, r, r) 
    $1 += 0x01
    $2 += $1
end
```

### Example

Writes 'EmmaChip8' to screen

```
alias letter v0
alias x v1
alias y v2
alias idx v3
alias delta 5 ;char width
alias len 9

data text "EmmaChip8"

loop
    read_data!(text, idx)
    draw_ascii!(x, y, letter)
    x += delta
    idx += 1
    if eq(idx, len) break
again

wait_for_key(v0)
```