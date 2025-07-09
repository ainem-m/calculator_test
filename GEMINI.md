# Project: CLI Calculator in Rust

This document outlines the step-by-step plan for building a command-line interface (CLI) calculator application using Rust. The goal is to create a simple, yet robust, calculator that can perform basic arithmetic operations.

This plan is designed to be executed by an AI assistant.

## Development Plan

### 1. Project Setup
- **Action:** Initialize a new Rust project using Cargo.
- **Command:** `cargo new calculator_cli`
- **Verification:** Confirm that `Cargo.toml` and `src/main.rs` are created successfully.

### 2. Implement Addition
- **File:** `src/main.rs`
- **Action:** Write a simple function to add two numbers. For now, use hardcoded values.
- **Goal:** Verify the basic program structure and compilation.
- **Verification:** Run `cargo run` and check that the correct sum is printed to the console.

### 3. Implement Subtraction, Multiplication, and Division
- **File:** `src/main.rs`
- **Action:** Add functions for subtraction, multiplication, and division.
- **Goal:** Complete the core arithmetic logic.
- **Verification:** Run `cargo run` and ensure all four operations produce the correct results.

### 4. Parse Command-Line Arguments
- **Action:** Add the `clap` crate to `Cargo.toml` for argument parsing.
- **Command:** `cargo add clap --features derive`
- **File:** `src/main.rs`
- **Action:**
    - Implement a struct to define the expected command-line arguments (e.g., `calc <number> <operator> <number>`).
    - Parse the arguments and use them as input for the calculator functions.
- **Goal:** Make the calculator interactive and usable from the command line.
- **Verification:**
    - Run `cargo run -- 10 + 5` and check for the correct output.
    - Test with other operations: `cargo run -- 10 - 5`, `cargo run -- 10 "*" 5`, `cargo run -- 10 / 5`.

### 5. Refactoring and Error Handling
- **Action:**
    - Organize the code into more logical modules (e.g., a separate module for calculation logic).
    - Implement robust error handling for invalid input, such as non-numeric arguments or division by zero.
- **Goal:** Improve code quality, maintainability, and user experience.
- **Verification:**
    - Run the calculator with invalid input (e.g., `cargo run -- 10 x 5`, `cargo run -- 10 / 0`, `cargo run -- ten + five`) and confirm that it prints user-friendly error messages instead of panicking.

### 6. (Optional) Add Tests
- **Action:** Create unit tests for the calculation logic.
- **Goal:** Ensure the correctness of the calculator's core functions.
- **Verification:** Run `cargo test` and confirm that all tests pass.