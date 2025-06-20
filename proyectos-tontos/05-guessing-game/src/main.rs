use std::io;
use std::cmp::Ordering;
use rand::random_range;

fn main() {
    let secret_number = random_range(1..=100);
    println!("🎯 Welcome to our guessing game!");    
    println!("Please guess a number between 1 and 100.");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("⚠️ Please enter a valid number.");
                continue;
            }
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is too small... Please try again"),
            Ordering::Greater => println!("Your guess is too large... Please try again"),
            Ordering::Equal => {
                println!("🎉 🎉 You win 🎉 🎉 ");
                break;
            }
        }
    }
}