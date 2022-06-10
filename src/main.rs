// Import the appropriate libraries into scope
use std::io;
use rand::Rng;
use std::cmp::Ordering;

// Function Main Declaration
fn main() {
    // println! is a macro, println is a function
    println!("Guess the Number v2!");
    
    // Generate secret number and assign to immutable variable
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");

    // Create mutable variable of type string
    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
