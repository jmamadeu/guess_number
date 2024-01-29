use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(0..100);

    println!("GUESS NUMBER GAME");

    loop {
        let mut attempt= String::new();
        println!("=======================================");
        println!("Enter your guess");
        io::stdin().read_line(&mut attempt).expect("Enter a value");

        let attempt: u32 = match attempt.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match attempt.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
        }

    }
}