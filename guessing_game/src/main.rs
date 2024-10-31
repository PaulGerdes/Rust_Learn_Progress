use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");


    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Generating Random number between 0 and 100

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess."); // user input with println!

        let mut guess = String::new(); // mutable so the value of the variable is changable
        // let apples = 5; immutable
        // let mut  bananas = 5; mutable


        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess); // getting user output with println!

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }

        }

    }



    // other variations syntaxes for the lines 13-15
    // io::stdin().read_line(&mut guess).read_line(&mut guess)
    // io::stdin().read_line(&mut guess).expect("Failed to read line");

    /*
        other example for an output of a simple value:

            let x = 5;
            let y = 10;
            println!("x = {x} and y + 2 = {}", y + 2);
     */
}


