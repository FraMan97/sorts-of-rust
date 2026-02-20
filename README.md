# sorts-of-rust

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/License-Apache_2.0-blue?style=for-the-badge&logo=apache)

`sorts-of-rust` is an educational library written in **Rust** that implements various sorting algorithms.
---

## Table of Contents

* [Key Features](#key-features)
* [Algorithms Implemented](#algorithms-implemented)
* [Project Structure](#project-structure)
* [Testing Strategy](#testing-strategy)
* [Usage](#usage)

---

## Key Features

* **Generic Implementation**: Every algorithm uses Generics (`<T: PartialOrd>`), allowing the library to sort any data type that supports comparison (integers, strings, etc.).
* **Performance Focused**: Includes optimized versions of algorithms (e.g., Bubble Sort with early exit) to ensure efficient execution.
* **Extensive Testing**: Robust codebase ensured by both internal unit tests and external integration tests.

---

## Algorithms Implemented

| Algorithm | Best Complexity | Average Complexity | Worst Complexity | Status |
| :--- | :--- | :--- | :--- | :--- |
| **Bubble Sort** | $O(n)$ | $O(n^2)$ | $O(n^2)$ | ✅ Completed |
| **Insertion Sort** | - | - | - | ⏳ Coming Soon |

---

## Project Structure

The project follows a clean, modular hierarchy:

* `src/lib.rs`: The primary entry point that manages public module exports.
* `src/algorithms.rs`: Defines the hierarchy and organization of the sorting algorithms.
* `src/algorithms/bubble.rs`: Contains the logic and unit tests for the Bubble Sort algorithm.

---

## Testing Strategy

This project adheres to Rust's best practices for testing:

* **Unit Tests**: Located within each source file to verify internal logic and edge cases (empty lists, single elements, ...).
* **Integration Tests**: Stored in the `/tests` directory to ensure the library works correctly when imported by an external project, testing public APIs with various data types.

---

## Usage

To test the library locally:

```bash
# Clone the repository
git clone https://github.com/YourUsername/sorts-of-rust.git
cd sorts-of-rust

# Run all tests (Unit and Integration)
cargo test

# Build the project in release mode
cargo build --release
```

## Integration

To use `sorts-of-rust` in your project, add the following dependency to your `Cargo.toml` file:

```toml
[dependencies]
sorts_of_rust = { git = "https://github.com/FraMan97/sorts-of-rust.git" }
```
