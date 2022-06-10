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
    
    println!("The secret number is: {}\n", secret_number);

    loop {
        // Create mutable variale of type string
        let mut guess = String::new();
        println!("Please input your guess.");

        // Read line from standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Transfer guess from string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
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