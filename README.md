# Rust Guessing Game 🦀

A simple command-line guessing game written in Rust.

This project is part of my journey learning Rust using _The Rust Programming Language_ (the official Rust book).  
I intentionally avoided using AI while building this to fully understand the fundamentals.

---

## 🧠 What This Project Does

- Prompts the user for their name
- Generates a random secret number between 1 and 10
- Repeatedly asks the user to guess the number
- Provides feedback if the guess is too small or too big
- Tracks the number of attempts
- Ends when the correct number is guessed

---

## 🛠️ Concepts Practiced

This project helped me understand and practice:

- Reading user input from `stdin`
- Working with mutable variables
- Pattern matching with `match`
- Error handling using `Result`
- Comparing values using enums (`std::cmp::Ordering`)
- Loops and control flow
- Basic usage of external crates (`rand`)

---

## ▶️ How to Run

Make sure you have Rust installed.  
If not, install it from: https://www.rust-lang.org/tools/install

Clone the repository and run:

```bash
cargo run
```
