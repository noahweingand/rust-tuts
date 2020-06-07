use std::io; // import input / output from standard lib
use rand::Rng; // for importing rng from rand dependency
use std::cmp::Ordering; // import compare, ordering from standard lib

// installed random crate in toml file.
// cargo fetches latest versions of everything from registry, which is a copy of data
// from Crates.io, where people in Rust ecosystem post their open sourced projects.

// cargo build will only build parts of project where code has changed if previously built.
// cargo update will ignore cargo.lock and update dependencies to latest version

fn main() {
    println!("Let's play guess a number!");
    let secret_number = rand::thread_rng().gen_range(1,101);
    loop {
        println!("Please input a number.");

        let mut guess = String::new();
        // let - variable declaration
        // variables are immutable by default.  'mut' makes a variable mutable.
        // '::new' indicates that new is an associated function of the String type.
        // An associated function is implemented on a type, so String in this case. (Static method) 

        io::stdin() //could be written std::io::stdin and remove import statement
            .read_line(&mut guess) //& = reference. References don't copy data into mem a lot times.
            .expect("Failed to read line :("); //prints error message if io::Result is an Err value.
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            }
        }
    }
}
