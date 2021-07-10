use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn edit_rect(&mut self) {
        let input = Rectangle::get_dim();
 
        self.width = input[0];
        self.height = input[1];

    }
    // Essential to accept reference of Rectangle otherwise borrowing will occour
    fn can_fit(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn set_rect() -> Rectangle {
        let input = Rectangle::get_dim();
 
        Rectangle {
             width: input[0],
             height: input[1],
         }
    }

    fn get_dim() -> [u32; 2] {
        // Take input
        let mut input: [String; 2] = [String::new(), String::new()]; 
        let msg: [&str; 2] = ["Enter width", "Enter height"];
        let mut i: usize = 0;
        while i <= 1 {
            input[i].clear();
            // With print! macro, you get prompt first then message.
            println!("{} : ", msg[i]);

            io::stdin().
            read_line(&mut input[i]).
            expect("Please enter a value greater than 0");
            // Added _ to eliminate warning of unused variable. Don't know what _ does here
            let _dim: u32 = match input[i].trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a whole number (>= 0)! Eg: 21");
                    continue
                    },
            };

            i += 1;
            print!("\n");
        }
        // Need to unwrap Result. Also proud of using shadowing
        let input: [u32; 2] = [input[0].trim().parse().unwrap(), input[1].trim().parse().unwrap()];
        input
    }
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };
    
    let mut new_rect = Rectangle {
        width: 15,
        height: 20,
    };

    println!("Hello, Rekt!. Your value is \n{:#?}", rect);
    println!("Your area is {}", rect.area());
    println!("A new Rekt appeared! Its value is \n{:#?}", new_rect);
    println!("Can OG Rekt fit new Rekt inside it? {}", rect.can_fit(&new_rect));
    println!("What a beautiful Sqr is that! {:#?}", Rectangle::square(15));
    println!("Edit a Rekt with your input!");
    
    new_rect.edit_rect();
    println!("New Rekt is now {:#?}", new_rect);
    println!("Create a new Rekt from scratch!");
    let new_new_rect = Rectangle::set_rect();
    println!("Your newest Rekt from scratch is {:#?}", new_new_rect);
}
