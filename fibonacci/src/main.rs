use std::io;

fn main() {
    println!("Welcome to the Fibonacci number genereator test program.");
    println!("Please enter a number");
    let mut to_convert = String::new();
    io::stdin()
        .read_line(&mut to_convert)
        .expect("failed to read line");
    let to_convert:i32 = match to_convert.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    let res = fibonacci(to_convert);
    println!("The {to_convert} Fibonacci number is: {res}");
}

fn fibonacci(x:i32) -> i32 {
    if x <= 1 {
        return x;
    } else {
        return fibonacci(x-1) + fibonacci(x-2);
    }
}