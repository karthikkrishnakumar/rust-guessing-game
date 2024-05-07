use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let mut how_many = String::new();
    println!("How many random numbers you want to guess?");
    let _ = io::stdin().read_line(&mut how_many).expect("Error reading input");
    let num_guesses: u32 = how_many.trim().parse().expect("Error parsing input");

    println!("Hey there, What's your name?");
    let mut first_name = String::new();
    let _ = io::stdin().read_line(&mut first_name).expect("Error reading input");

    let mut correct = Vec::new();
    for _ in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10))
    }
    println!("Correct numbers: {:?}", correct);

    println!("Hey {}, guess a number from 1-10:", first_name.trim());

    let mut guesses_made = 0;
    while guesses_made < num_guesses {
        let mut guess_number = String::new();
        let _ = io::stdin().read_line(&mut guess_number).expect("Error reading input");

        let guess_number: u32 = match guess_number.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error occurred: {}", err);
                continue;
            }
        };

        let message = match guess_number.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => "You guessed too high",
            Ordering::Less => "You guessed too low",
            Ordering::Equal => {
                println!("You win!");
                guesses_made += 1;
                continue;
            }
        };
        println!("{}", message);
    }
}
