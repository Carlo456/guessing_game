use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guessing game!!!");
    loop {
        println!("Please enter the number");
        let secret_number =rand::thread_rng().gen_range(1..=10);

        let mut guess: String = String::new();
        let error_msg = "There was an error";

        io::stdin()
            .read_line(&mut guess)
            .expect(error_msg);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small.."),
            Ordering::Greater => println!("Too big.."),
            Ordering::Equal => {
                println!("You win..");
                break;
            }
        }    
    }
    
}
