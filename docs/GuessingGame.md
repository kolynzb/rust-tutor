# Rust Guessing Game. CHapter 2

- We’ll implement a classic beginner programming problem: a guessing game. Here’s how it works: the program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low ortoo high. If the guess is correct, the game will print a congratulatory message and exit.

```rs
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
    println!("You guessed: {}", guess);
}

```

- To obtain user input and then print the result as output, we need to bring the io (input/output) library into scope. The io library comes from the
  standard library (which is known as std). `use std::io;`
  - By default rust brings in a few values from the prelude.If you need to include something thats not in the prelude import it.
- `fn main()` is the entry point of the program.
- `let mut guess = String::new();` this creates a place to store user input.
