/*
 * chapter 2.0: Programming a Guessing game
 * 20240903
 * 20240913: addied 'colored' crate
 */

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your gues.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // 20240913
            // Ordering::Less => println!("Too small!"),
            // Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                // 20240913
                // println!("You win!");
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

