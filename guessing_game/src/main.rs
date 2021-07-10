use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // Explore if you can nest a match inside for Err where the value is "quit" to provide an avenue to quit the game
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // Err(_) => {
            //         println!("Error Input: {}", &guess[..]);
            //         match &guess[..] {
            //             "quit" => break,
            //             _ => continue,
            //         }
            //     },
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
        };
    }
}


