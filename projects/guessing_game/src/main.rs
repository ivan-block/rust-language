use std::io;
//standard input library
use std::cmp::Ordering;
use rand::Rng;
// defines traits that random number generators implement, and this trait must be in scope for us to use those methods


fn main() {
    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Your seret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); //mutable empty string

    //receive user input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read lines");

    loop {
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}