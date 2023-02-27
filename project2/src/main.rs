use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    println!("You guessed: {guess}");

    match guess.as_str().cmp(&secret_number.as_str()) {
        Ordering::Less => println!("Small"),
        Ordering::Greater => println!("Great"),
        Ordering::Equal => println!("Equal")
    }
}
