/*
    Project:        guessing_game
    File:           main.rs
    Date Created:   04/02/2022
    Author:         Matthew Colson
    Purpose:        Simple guess the number game.
*/

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("<---  NUMBER GUESSING GAME!  --->");

    // Generates a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut times_guessed: u32 = 0;

    // Loop until user guesses correct number
    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        // Reads in user input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Parses user input string into a unsigned 32 bit int 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter numbers!");
                continue;
            },
        };

        println!("You guessed: {}", guess);
        times_guessed += 1;
        
        // Compares guess from random number and alerts user of result
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => {
                println!("You win!");
                println!("You got in {} guesses!!!", times_guessed);
                println!("Thanks for playing!!!");
                break;
            },
        }
    }
}
