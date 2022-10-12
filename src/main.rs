use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Let's start a guessing game!");
    println!("Please provide a number from 1 to 100.");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Bravo! You guessed the number!");
                break;
            },
        };
    }


}