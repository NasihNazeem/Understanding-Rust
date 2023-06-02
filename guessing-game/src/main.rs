use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*; 

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is: {}", secret_number);

    loop {
        
        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
            Err(_) => {println!("invalid guess");continue; },
            Ok(num) => num,  
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Equal => {println!("{}","You win!".green()); break;},
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
        }
    }
}
