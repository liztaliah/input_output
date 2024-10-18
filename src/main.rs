//Input Output
//Liz Hardee Oct 12 2024

// A simple input output program
// Reads the name and greets the User

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your name: ");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Hello, {}", input.to_uppercase());
        },
        Err(e) => println!("Sorry, I didn't catch that: {}", e)
    }
}