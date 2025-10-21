fn main() {
    println!("Guess the number!");
    let secret_number: u32 = (rand::random::<u32>() % 100) + 1;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Please type a valid number (1-100)!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        use std::cmp::Ordering;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
