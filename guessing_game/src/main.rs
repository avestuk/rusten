use core::fmt;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl fmt::Display for Guess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Guess {
    pub fn new(i: i32) -> Guess {
        // This is a great construct!
        if !(0..=100).contains(&i) {
            panic!("Guess must be between 1 and 100, got: {}", i)
        }
        Guess { value: i }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);

    println!("Please enter a guess between 1 and 100");

    let mut guess_count = 0;
    loop {
        println!("Please enter your guess:\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: Guess = match guess.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        guess_count += 1;

        println!("You guessed {}", guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("You guessed too low"),
            Ordering::Greater => println!("You guessed too high"),
            Ordering::Equal => {
                println!("You got it right!");
                println!("You used {} guesses", guess_count);
                break;
            }
        }
    }
}
