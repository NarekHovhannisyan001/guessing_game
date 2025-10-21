# Guessing Game

A simple Rust command-line game where you guess a number between 1 and 100.

## How to Run

1. Make sure Rust is installed: [Install Rust](https://www.rust-lang.org/tools/install)
2. Clone the repository or download the code.
3. Open a terminal in the project folder and run:

```bash
cargo run
```

## How to Play

- The program picks a secret number between 1 and 100.
- Enter your guess and press Enter.
- The program will tell you if your guess is too low, too high, or correct.
- Keep guessing until you find the number.

## Example Usage

```
Guess the number!
Please input your guess.
> 50
You guessed: 50
Too high!
> 25
You guessed: 25
Too low!
> 37
You guessed: 37
You win!
```