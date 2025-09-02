# Rust Learning Journey – Christopher Tablas

This repository contains my exercises, experiments, and small projects while learning Rust through the interactive Brown CS version of *The Rust Programming Language*. This is meant for me to keep track of my progress.

## Why the Brown version?
The [Brown CS edition of The Rust Programming Language](https://rust-book.cs.brown.edu/) is an experimental fork of the official Rust Book that adds interactive quizzes and highlighting to reinforce learning as you go. I highly recommend it to new learners.

## Repository Structure
This is organized as a Cargo workspace. Each folder under the root is an independent binary crate corresponding to a chapter or a specific exercise.

Examples:
- `chapter_one_rust_examples/hello_world` – first "Hello, world!" program
- `chapter_two_guessing_game` – number guessing game from Chapter 2
- `convertTempRust` – Fahrenheit to Celsius converter
- `nthfib` – nth Fibonacci number generator
- `twelve_days_of_christmas` – prints the carol with loops and arrays

## How to Run

Build the entire workspace:
```bash
cargo build
```
Run any specific crate from the root:
```bash
cargo run -p guessing_game
cargo run -p nthfib
```

## Notes

Each crate handles its own dependencies (for example, `rand` is only in the guessing-game crate).

The workspace uses `resolver = "2"` for modern and more predictable feature resolution.

This code is experimental and primarily for learning — not intended for production use.