# bspl
Bit-Shift-Print Loop

## Introduction

`bspl` is a [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop)
for practising bit-wise operations. Aimed at undergraduates in CS/SE/ECE or
anyone that is interested in learning binary level manipulations.

## Design

`bspl` is split into three phases: the lexer, parser, and evaluator.

The lexer is only responsible for creating a token stream from the REPL input.
It does not concern itself with the semantics of the language. It is only interested
in understanding the syntax and classifying the scanned tokens.

To this end, the parser is responsible for pruning and creating a structure out
of the raw token stream. As a result, the parser contains stricter rules and more
edge cases to deal with it.  Some of the heavier token analysis work of the parser
may have been alleviated had these rules been shifted earlier to the lexer.
utilizes the Shunting Yard Algorithm, the parser will create


## References

[Bit Twiddling Hacks](http://graphics.stanford.edu/~seander/bithacks.html)

[Hacker's Delight](http://www.hackersdelight.org/)

[awesome-bits](https://github.com/keonkim/awesome-bits)

[Rsut Bitwise Operations](https://rosettacode.org/wiki/Bitwise_operations#Rust)

[Precedence](http://introcs.cs.princeton.edu/java/11precedence/)

[Shunting Yard Algorithm Python Implementation](http://rosettacode.org/wiki/Parsing/Shunting-yard_algorithm#Python)

[An Overview of Lexers and Parsers](http://savage.net.au/Ron/html/graphviz2.marpa/Lexing.and.Parsing.Overview.html#My_Rules-of-Thumb_for_Writing_Lexers%2FParsers)

## Roadmap

Learn how parsers work

Support for memory addressing

Base Conversions: define a standard method e.g. 16:10 AF (Convert hexadecimal value to decimal)

Operators: + - * / & | ~

Explain why this offers a richer experience compared to the Python REPL.

Add Octal to supported bases

Support for equality: ==
