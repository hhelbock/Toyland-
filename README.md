# Toyland

Toyland is a small, self-designed programming language with its own compiler,
written from scratch in Rust. It compiles directly to x86-64 assembly (Linux),
which is then assembled and linked into a real, native executable — no
interpreter, no virtual machine, no external runtime.

This is a personal learning project. The goal was to understand, hands-on, how
a compiler actually works end to end — lexing, parsing, building an AST, and
generating real machine code — rather than to build a production-ready
language.

## Why "Toyland"?

The name started as a typo (`cargo new toyland` instead of `toylang`) and
stuck.

## What Toyland can do right now

- Static typing with explicit type annotations (`Number`, `Decimal`, `String`
  exist as types; `Decimal`/`String` codegen is not implemented yet — see
  Roadmap)
- Variables: declaration and reassignment
- Integer arithmetic: `+`, `-`, `*`, `/`
- Comparisons: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Control flow: `If` / `Otherwise`, `While`
- Functions with a single, no-argument entry point (`Func main(): Number { ... }`)
- Compiles to real x86-64 assembly and runs as a native Linux executable

## Example

```
Func main(): Number {
    x As Number = 0;
    While (x < 3) {
        x = x + 1;
    }
    Return x;
}
```

This compiles, runs, and exits with code `3`.

## Language notes

- Keywords must be capitalized (`Func`, `If`, `While`, `Return`, `As`,
  `Number`...). Lowercase words are always treated as identifiers — this is a
  deliberate design choice, not an inconsistency.
- `=` is assignment, `==` is comparison — no C-style ambiguity between them.
- `If`/`While` conditions require parentheses; bodies require braces.
- `End` is Toyland's `break`; `Continue` works as in most C-family languages.
  

## How it works

Toyland source goes through four stages, all hand-written in Rust:

1. **Lexer** — turns raw source text into a stream of tokens.
2. **Parser** — a recursive-descent parser that turns tokens into an AST.
3. **Codegen** — walks the AST and emits x86-64 assembly (Intel syntax, NASM
   dialect).
4. **Assemble & link** — the generated `.asm` file is handed to `nasm` and
   `ld` (both external, standard tools) to produce a native executable.

## Building and running

Requires `rustc`/`cargo`, `nasm`, and `ld` (all standard on most Linux
distros).

```bash
cargo run          # runs the compiler on the hardcoded test program,
                    # producing main.asm

nasm -f elf64 main.asm -o main.o
ld main.o -o main

./main
echo $?             # prints the program's return value as its exit code
```

## Roadmap / not yet supported

Toyland is an ongoing learning project. Notably missing, on purpose, for now:

- Function parameters and multi-argument function calls
- `Decimal` and `String` codegen (the types exist and parse correctly; the
  compiler doesn't yet generate assembly for them)
- I/O (`print`, `input`)
- A module / `#include` system for splitting code across files
- Taking a source file as input, rather than a hardcoded string in `main.rs`
- 'End' and 'Continue'

## Motivation

Built as a hands-on way to learn Rust deeply (ownership, recursive data
structures, pattern matching) and to understand how compilers work from first
principles — lexing, parsing, and x86-64 code generation — rather than
treating any of it as a black box.
