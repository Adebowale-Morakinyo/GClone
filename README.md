# 🕵️‍♂️ Minigrep

A simple command-line tool written in Rust for searching text within files — inspired by the classic Unix `grep` utility.

---

## 🚀 Features

- Command-line interface for searching text in files
- Built using idiomatic Rust
- Designed for learning and extendability
- Good foundation for exploring:
  - Ownership and borrowing
  - Result and error handling
  - File I/O
  - Modular Rust code

---

## 🛠 Usage

```bash
# Build the project
cargo build

# Run the search
cargo run -- <search_term> <filename>

# Example
cargo run -- to poem.txt
