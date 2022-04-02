use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("<---  NUMBER GUESSING GAME!  --->");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut times_guessed: u32 = 0;

    loop {
        println!("Please input your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please only enter numbers!");
                continue;
            },
        };

        println!("You guessed: {}", guess);
        times_guessed += 1;
        
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
