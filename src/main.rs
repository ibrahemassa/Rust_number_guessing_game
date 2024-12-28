use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!");
    let secret_number: u32 = rand::thread_rng().gen_range(1..100);
    loop{
        println!("Enter your guess:");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Invalid input!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("You should enter a number!");
                continue;
            },
        };

        println!("You've guessed: {guess}");
        match guess.cmp(&secret_number){
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Less => println!("Too Small!!"),
            Ordering::Equal => {
                println!("Correct!!");
                break;
            },
        }
    }
}
