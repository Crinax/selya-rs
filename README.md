# Selya
SELYA (Special Esoteric Language for Young and Adult) - it's esoteric language for everyone based on hex :D

## Syntax

So, probably, all of you already know, that in most programming languages such string as `0xA2` - is it hexadecimal number

Well, Selya use it as often as it can :D

Every each hexadecimal number that is used you, is written to corresponding cell. If you write number greater than `0xFFFF` or lower than `0x0000`, it calls the error `[Selya::Memory::InvalidValue] Invalid value`

Default the carriage installed to first position, but you can change it with operator `->` or `<-`

Attention, if you try to use operator `<-` in first position or use operator `->` in last position, it calls the error `[Selya::Memory::OutOfRange] Carriage out of boundaries of memory`

### Operators

It is strange, that I begin this chapter after two operators description, but nevertheless

So, Selya has next operators:

1. `-->` and `<--` - shifts carriage of memory in the direction of the arrow
2. `[+]` - add the next value to the current cell (if cell is overflowed, calls the error `[Selya::Memory::Overflow] Cell is overflowed`)
3. `[^]` - addition modulo 2 of the next value with the current value in the cell
4. `[>]` and `[<]` - shift all words in the direction of the arrow

In addition to Selya operators, it also has commands (not released):

1. I don't wanna talk about it, try to find it by yourself :D
2. `zen` - write zen of the Selya

### Rules

1. First value that you write - memory size. Memory size can be greater than `0x1` and lower than `0xFFFF`, another values call error `[Selya::Memory] Invalid memory size`
2. You cannot use undefined characters, it calls the error `[Selya::Parser::UnrecognizedSymbol] Unrecognized symbol`
666. You can get the satanic power by 6 words :D
