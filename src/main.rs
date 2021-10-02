use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // :: -> refers to an associated f'n or handle
    let secret_num = rand::thread_rng().gen_range(1..=100);

    // {} -> placeholders for printing the ensuing variables
    // println!("The secret number is: {}", secret_num);

    loop {
        println!("Please input your guess: ");

        // mut -> makes variable mutable (immutable by default)
        let mut guess = String::new();

        // read_line() -> returns Result type enum (Result === "Ok" || "Err")
        io::stdin()
            // &mut -> pointers are also immutable by default
            .read_line(&mut guess)
            .expect("Failed to read line...");

        // parse() -> returns Result enum
        // here we shadow the previous value of guess with new u32 type
        let guess: u32 = match guess.trim().parse() {
            // we can handle the Result explicitly (instead of using expect())
            // match -> executes depending on which Result returned from parse()
            Ok(num) => num,
            // (_) -> is a catchall value (i.e. handle any error)
            // continue -> move on to next loop iteration
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match -> works like a switch statement based on Ordering enum
        // Ordering === "Less" || "Equal" || "Greater"
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
