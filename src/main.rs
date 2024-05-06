use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("{correct}");
    println!("Hey there, What's your name.?");
    let mut first_name = String::new();
    let _ = io::stdin()
        .read_line(&mut first_name)
        .expect("Error reading input");

    loop {
        println!("Hey {}{}", first_name, "guess a number from 1-10:");
        let mut guess_number = String::new();
        let _ = io::stdin()
            .read_line(&mut guess_number)
            .expect("Error reading input");

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error occured : {err}");
                continue;
            }
        };

        let message = match guess_number.cmp(&correct) {
            Ordering::Greater => "You guessed too high",
            Ordering::Less => "You guessed too low",
            Ordering::Equal => {
                println!("wins the game");
                break;
            }
        };
        println!("{}", message);
    }
}
