use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number : u32= rand::rng().gen_range(1..=100); 
    println!("Welcome to the guessing game!");
   
   loop {
        println!("Please enter a number between 1 and 100:");
    let mut guess : String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num ,
        Err(_) => {
            println!("Please enter a valid number.");
            continue;
        }
    };
    
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
