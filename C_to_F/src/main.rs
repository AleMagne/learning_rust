use std::io;

fn main() {
    println!("Welcome to the Celsius to Farenheit conversion test program.");
    println!("Please enter a number");
    let mut to_convert = String::new();
    io::stdin()
        .read_line(&mut to_convert)
        .expect("failed to read line");
    let to_convert:i32 = match to_convert.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("Please entre a number not a string");
            0
        },
    };
    let result = c_to_f_converter(to_convert);
    println!("the result of the conversion is {result}");
}

fn c_to_f_converter(x:i32) -> i32 {
    (x * 9/5) +32
}
//(°C × 9/5) + 32 = 89,6 °F