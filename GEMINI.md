# Project: CLI Calculator in Rust

This document outlines the step-by-step plan for building a command-line interface (CLI) calculator application using Rust. The goal is to create a simple, yet robust, calculator that can perform basic arithmetic operations.

This plan is designed to be executed by an AI assistant.

## Development Plan

### 1. Project Setup (Completed)
- **Action:** Initialize a new Rust project using Cargo.
- **Command:** `cargo new calculator_cli`
- **Verification:** Confirmed that `Cargo.toml` and `src/main.rs` are created successfully.

### 2. Implement Addition (Completed)
- **File:** `src/main.rs` (logic moved to `src/lib.rs`)
- **Action:** Wrote a function to add two numbers.
- **Goal:** Verified the basic program structure and compilation.
- **Verification:** Confirmed correct sum is printed to the console.

### 3. Implement Subtraction, Multiplication, and Division (Completed)
- **File:** `src/lib.rs`
- **Action:** Added functions for subtraction, multiplication, and division.
- **Goal:** Completed the core arithmetic logic.
- **Verification:** Confirmed all four operations produce the correct results.

### 4. Parse Command-Line Arguments (Completed)
- **Action:** Added the `clap` crate to `Cargo.toml` for argument parsing.
- **Command:** `cargo add clap --features derive`
- **File:** `src/main.rs`
- **Action:**
    - Implemented a struct to define the expected command-line arguments (e.g., `calc <number> <operator> <number>`).
    - Parsed the arguments and used them as input for the calculator functions.
- **Goal:** Made the calculator interactive and usable from the command line.
- **Verification:**
    - Confirmed correct output for `cargo run -- 10 + 5` and other operations.

### 5. Refactoring and Error Handling (Completed)
- **Action:**
    - Organized the code into logical modules (`src/lib.rs` for calculation logic).
    - Implemented robust error handling for invalid input (non-numeric arguments, division by zero, invalid operator).
- **Goal:** Improved code quality, maintainability, and user experience.
- **Verification:**
    - Confirmed user-friendly error messages for invalid input (e.g., `cargo run -- 10 x 5`, `cargo run -- 10 / 0`, `cargo run -- ten + five`).

### 6. Add Tests (Completed)
- **Action:** Created unit tests for the calculation logic in `src/lib.rs`.
- **Goal:** Ensured the correctness of the calculator's core functions.
- **Verification:** Confirmed all tests pass with `cargo test`.