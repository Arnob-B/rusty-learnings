# 🎯 Guess the Number - Rust CLI Game

A simple command-line game written in **Rust** where you try to guess a randomly generated number between 1 and 100. The game will guide you whether your guess is too low, too high, or correct!

## 🛠️ Features

- Random number generation using the `rand` crate.
- Robust input handling with user-friendly error messages.
- Infinite loop until the correct number is guessed.
- Simple and clear game logic.
- Great starting point for Rust beginners.

---

## 🧠 How It Works

1. The program greets the user.
2. It generates a random number between **1 and 100**.
3. It enters a loop asking for the user's guess.
4. If the input is not a valid number, it asks again.
5. Compares your guess to the secret number and gives feedback:
   - 🔽 "Too small!"
   - 🔼 "Too big!"
   - ✅ "You win!" (and exits the loop)

---

## 📄 Example Output

```bash
Guess the number!!
Plse input your guess : 
50
Too big!
Plse input your guess : 
25
Too small!
Plse input your guess : 
37
You win!
```

---

## 📦 Dependencies

- [`rand`](https://crates.io/crates/rand) - for random number generation

To add `rand`, ensure your `Cargo.toml` includes:

```toml
[dependencies]
rand = "0.4"
```

---

## 📚 Learning Rust?

This game is based on the [official Rust book’s tutorial](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html). It's a great first project to understand variables, loops, input/output, and basic pattern matching.

---

## ✍️ Author
[@Arnob-B](https://github.com/Arnob-B/)
