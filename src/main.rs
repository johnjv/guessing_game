extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("A GUESSING GAME");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("please input a number between 1 and 100:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read input");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("input is not a number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal   => {
                println!("correct!");
                break;
            }
        }
    }
    
}
