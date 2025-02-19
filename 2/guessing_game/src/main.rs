extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    println!("Guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: '{}'", secret_number);

    loop
    {

    println!("Please enter your guess: ");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {println!("you have to enter number!!"); continue;},
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => {println!("You win!"); break;},
        Ordering::Greater => println!("Too big"),
    }
    }
}
