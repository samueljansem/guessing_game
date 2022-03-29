use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = thread_rng().gen_range(1..101);
    println!("{}", secret_number);

    loop {
        let mut guess = String::new();
        println!("Please input your guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Smaller!"),
            Ordering::Greater => println!("Bigger!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
