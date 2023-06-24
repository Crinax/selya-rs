# Selya
SELYA (Special Esoteric Language for Young and Adult) -- it is esoteric language for everyone based on hex :D

## Installation

```sh
cargo install selya --features clap
```

## Syntax

So, probably, all of you already know, that in most programming languages such string as `0xA2` -- is it hexadecimal number

Well, Selya use it as often as it can :D

Every each hexadecimal number that is used you, is written to corresponding cell.

Default the carriage installed to first position, but you can change it with operator `-->` or `<--`

### Operators

It is strange, that I begin this chapter after two operators description, but nevertheless

So, Selya has next operators:

1. `-->` and `<--` -- shifts carriage of memory in the direction of the arrow;
2. `[+]` -- add the next value to the current cell (if cell is overflowed;
3. `[^]` -- addition modulo 2 of the next value with the current value in the cell;
4. `[>]` and `[<]` -- rotate memory in the specified direction;

In addition to Selya operators, it also has commands (not released):

1. I don't wanna talk about it, try to find it by yourself :D
2. `zen` -- write zen of the Selya

### Rules

1. First value that you write -- memory size. Memory size can be greater or equal `0x0` and lower than `0xFFFF`, another value cannot be recognized by parser
666. You can get the satanic power by 6 words :D

### Errors

All the errors have the same kind as `[Selya::Pipeline::{kind}]: {description}`. Possible kinds of errors:
1. `IoError` -- selya interpreter cannot read your file;
2. `ParserError` -- cannot parse your file. It seems when you for examples write more than 4 digits in the number;
3. `Memory::Overflow` -- in the cell were written number more than `0xFFFF`;
4. `Memory::OutOfRange` -- appear when you try shifts on the boundaries of memory;
5. `Interpreter::UsingBinaryAsUnary` -- after binary operator (`[+]` and `[^]`) there is no number.
