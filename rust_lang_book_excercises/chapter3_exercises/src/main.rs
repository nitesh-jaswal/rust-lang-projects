use std::io;
use std::process;

enum MenuChoice{
    Pingala(u16),
    FtoC(f64),
}

impl MenuChoice {
    fn get_input(&self) -> MenuChoice {
        let mut input: String;
        
        match self {
            MenuChoice::FtoC => {
                loop {
                    input = String::new();

                    println!("Enter temperature in Faranheit:");
                    io::stdin()
                    .read_line(&mut input)
                    .expect("Enter a valid temperature!");
                    
                    let temp: f64 = match input.trim().parse() {
                                        Ok(num) => num,
                                        Err(_) => {
                                            println!("Please enter a valid temperature in decimal format (Eg: 32.0)");
                                            continue;
                                    }
                                };
                    return MenuChoice::FtoC(temp);
                    };
                }
            MenuChoice::Pingala => {
                loop {
                    // If I don't do this and initialize it outside loop, if I give a wrong input like -1, the value never get overwritten
                    input = String::new();
                    println!("Enter which number in the Pingala Series you want to know:");
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Enter a valid number!");

                    let num: u16 = match input.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {println!("Please enter a number greater than or equal to 0"); continue}
                    };
                    
                    return MenuChoice::Pingala(num);
                }
            
            },
        }
    }
}

fn main() {
    println!("*+*+*+*+*+*+*Hi! Welcome to Rustyland!*+*+*+*+*+*+*\n\n");
    loop {
        let mut choice = String::new();

        println!("Choose one of the following options:\n1. F to C Temperature Conversion.\n2. Nth number in Pingala Series\n3. Lyrics of some song.\n4. Quit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Could not read line");
        
        let choice: u8 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input a number between 1 to 4");
                continue
            }
        };

        let choice: MenuChoice = match choice {
            1 => {MenuChoice::Pingala(0),
                let temp = get_input();
                f_to_c_temperature()
            },
            2 => {pingala_series()},
            3 => {println!("Function not implemented. Try a different option."); continue},
            4 => {println!("Thank you! Quitting..."); process::exit(1)},
            _ => {println!("Please choose from 1 to 4"); continue}

        }
    }
}

fn f_to_c_temperature() {
    // Code to take input
    let t: f64 = get_input("F_TO_C").1; 
    
    println!("\nConverted temperature is: {} C\n", ((t - 32.0) * 5.0/9.0));
}

fn pingala_series() {
    // Code to take input
    let num: u16 = get_input("PINGALA").0;

    // Pingala Code starts here
    let mut sum: u64;
    if num == 0 {
        sum = 0;
    }
    else if num <= 2 {
        sum = 1;
    }
    else {
        sum= 2;
        let mut swap: u64;
        let mut prev: u64 = 1;
        for _i in 3..num as u64 {
            swap = sum;
            sum = sum + prev;
            prev = swap;
        }
    }

    println!("\n{}th Pingala number is {}\n", num, sum);
}


fn get_input(in_type: &str) -> (u16, f64) {
    
    let mut input;
    let mut flag = false;

    if in_type == "F_TO_C" {
        let mut tem: f64;

        loop {
            input =  String::new();
            println!("Enter temperature in Faranheit:");
            io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid temperature!");
    
            tem = match input.trim().parse() {
                Ok(num) => {flag = true; num},
                Err(_) => {
                    println!("Please enter a valid temperature in decimal format (Eg: 32.0)");
                    continue;
                    }
            };

            if flag {
                return (0, tem);
            }
        }
    }
    else if in_type == "PINGALA" {
        let mut num: u16;

        loop {
            // If I don't do this and initialize it outside loop, if I give a wrong input like -1, the value never get overwritten
            input = String::new();
            println!("Enter which number in the Pingala Series you want to know:");
            io::stdin()
                .read_line(&mut input)
                .expect("Enter a valid number!");
    
            num = match input.trim().parse() {
                Ok(num) => {flag = true; num},
                Err(_) => {println!("Please enter a number greater than or equal to 0"); continue}
            };
    
            if flag {
                return (num, 0.0);
            }
        }
    }
    
    return (0, 0.0);
}