use rand::Rng;

use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

pub struct Guess {
    value: i32,
}

#[derive(Debug)]
pub enum GuessError {
    ValueTooLow,
    ValueTooHigh,
    InvalidNumber,
    Unknown(ParseIntError),
}

impl Guess {
    pub fn new(value: &str) -> Result<Guess, GuessError> {
        match value.trim().parse() {
            Ok(value) => match value {
                i32::MIN..0 => Err(GuessError::ValueTooLow),
                0..=100 => Ok(Guess { value }),
                101..=i32::MAX => Err(GuessError::ValueTooHigh),
            },
            Err(e) => Err(GuessError::Unknown(e)),
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match Guess::new(&guess) {
            Ok(guess) => guess,
            Err(e) => {
                println!("Error: {e:?}");
                continue
            },
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
