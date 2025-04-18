use rand::prelude::*;
use std::{cmp::Ordering, io};

// Guessing Game
pub fn guessing_game() {
    println!("Welcome to Guessing Game!");

    // Step 1: Computer guess a random number and
    // Step 2: stores in memory
    let mut rng = rand::rng();
    let computer_rand_number: i32 = rng.random_range(1..=100);
    // println!("{computer_rand_number}");

    println!("Guess a Number:");

    loop {
        // Step 3: ask the user to guess the number stored in the memory
        let mut guess = String::new();
        // Step 4: Users inputs number
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to get input");

        let guess = guess.trim();

        // Step 5: Check if the user wants to stop
        if guess.eq_ignore_ascii_case("stop") {
            println!("Game stopped by user.");
            break;
        }

        // Step 5a: Convert user input from string to Integer.
        let parsed_guess: i32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a valid number or type 'stop' to exit.");
                continue;
            }
        };

        // Step 6: If number is correct inform user wins, else inform user if guess lower or higher
        let cmp_guess = parsed_guess.cmp(&computer_rand_number);
        match cmp_guess {
            Ordering::Less => println!("Your Guess is less! Guess Again"),
            Ordering::Greater => println!("Your Guess is greater! Guess Again"),
            Ordering::Equal => {
                println!("Your Guess is correct. You win!");
                break;
            }
        }

        // Step 7: Start from step 1
    }
}
