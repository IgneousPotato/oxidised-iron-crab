use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number bitch ;)");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("Secret number is {secret_number} bitch ;)");

    loop {
        println!("Input your guess bitch ;)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line bitch ;(");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number bitch ;(");
                continue
            }
        };
        
        println!("You guessed: {guess} bitch ;)");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small bitch ;("),
            Ordering::Greater => println!("Too big biiitttchhh ;)"),
            Ordering::Equal => {
                println!("Perfect bitch ;)");
                break;    
            }
        }
    }
}
