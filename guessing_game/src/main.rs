use std::{io, error::Error};
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    loop{
        println!("Please input your guess.");

        let mut guess = String::new();
        let a: Result<(), u128> = Ok(());
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(e) => match e.kind() {
                std::num::IntErrorKind::Empty => todo!(),
                std::num::IntErrorKind::InvalidDigit => todo!(),
                std::num::IntErrorKind::PosOverflow => todo!(),
                std::num::IntErrorKind::NegOverflow => todo!(),
                std::num::IntErrorKind::Zero => todo!(),
                _ => todo!(),
            },
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
