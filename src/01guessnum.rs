use std::io;

use rand::Rng;

fn main() {
    println!("Guess the number");
    println!("Please input your number");

    let secret = rand::rng().random_range(1..=100);
    
    loop {
        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess:u32 = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => {
                println!("Please input a valid number");
                continue;
            }
        };

        println!("you guessed: {}",guess);
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Equal => {
                println!("You got it!");
                break;
            }
        }
    }


    println!("The secret number is: {}", secret);
}