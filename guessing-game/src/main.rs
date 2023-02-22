use std::io;
use std::cmp::Ordering;
use rand::Rng;

// my first rust application

fn main() {
    let secret_number= rand::thread_rng().gen_range(1..=100);
    println!("the number is {secret_number}");
    println!("guess the number!");

    loop{
        println!("please enter your guess");

        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please entre a number not a string");
                continue;
            },
        };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct, you won");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
    
}