use std::{cmp::Ordering, io::stdin};

use rand::Rng;

fn main() {
    println!("Guess the number");
    let random_number = generate_random_number();
    loop {
        println!("Input the number");
        let maybe_guessed_number = try_read_guessed_number();

        let guessed_number;

        match maybe_guessed_number {
            Some(num) => guessed_number = num,
            None => {
                println!("Invalid input");
                continue;
            }
        }

        // println!("You guessed: {guessed_number}");

        let ord = random_number.cmp(&guessed_number);

        match ord {
            Ordering::Less => {
                println!("Too big")
            }
            Ordering::Greater => {
                println!("Too small")
            }
            Ordering::Equal => {
                println!("You guessed");
                break;
            }
        }
    }
    println!("Game finished");
}

fn try_read_guessed_number() -> Option<usize> {
    let mut guess = String::new();

    let input = stdin();
    input
        .read_line(&mut guess)
        .expect("failed to read from stdin");

    guess.trim().parse().ok()
}

fn generate_random_number() -> usize {
    let mut rng = rand::thread_rng();
    let y: usize = rng.gen_range(0..100);
    y
}
