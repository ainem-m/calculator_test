# CLI Calculator in Rust

This is a simple command-line interface (CLI) calculator application built with Rust.

## Features

- Performs basic arithmetic operations: addition, subtraction, multiplication, and division.
- Handles both integer and floating-point numbers.
- Provides clear error messages for invalid input, such as non-numeric values or division by zero.
- Includes a test suite to ensure calculation accuracy and robustness, covering edge cases like large number overflows.

## Usage

You can run the calculator from the command line by providing two numbers and an operator.

### Syntax

```sh
cargo run -- <value1> <operator> <value2>
```

- `<value1>`: The first number.
- `<operator>`: The arithmetic operator (+, -, *, /).
- `<value2>`: The second number.

**Note:** When using the multiplication operator (`*`), you may need to enclose it in quotes to prevent your shell from interpreting it as a wildcard.

### Examples

#### Addition
```sh
cargo run -- 10 + 5
# Output: 10 + 5 = 15
```

#### Subtraction
```sh
cargo run -- 10 - 5
# Output: 10 - 5 = 5
```

#### Multiplication
```sh
cargo run -- 10 "*" 5
# Output: 10 * 5 = 50
```

#### Division
```sh
cargo run -- 10 / 2.5
# Output: 10 / 2.5 = 4
```

#### Error Handling
```sh
cargo run -- 10 / 0
# Output: Error: Division by zero
```

## Building from Source

1.  **Clone the repository:**
    ```sh
    # git clone <repository-url>
    # cd calculator_cli
    ```

2.  **Build the project:**
    ```sh
    cargo build --release
    ```
    The executable will be located at `target/release/calculator_cli`.

3.  **Run the tests:**
    To ensure everything is working correctly, run the built-in unit tests.
    ```sh
    cargo test
    ```