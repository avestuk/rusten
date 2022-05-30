use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("Please enter a guess between 1 and 100");

    let mut guess_count = 0;
    loop {
        println!("Please enter your guess:\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        guess_count += 1;

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
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
