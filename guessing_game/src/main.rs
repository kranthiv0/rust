use rand::Rng;

use std::cmp::Ordering;
fn main() {
    loop {
        println!("Guess the number!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        println!("The secret number is : {secret_number}");
        println!("Please input your number:");
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        println!("You guessed the number : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}

