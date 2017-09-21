// This is a program to convert Fahrenheit to Celsius and vice-versa.
use std::io;

fn fahrenheit_to_celsius(ftemp: f64) -> f64 {
    let ctemp = (ftemp - 32.0) / 1.8;
    ctemp
}

fn celsius_to_fahrenheit(ctemp: f64) -> f64 {
    let ftemp = ctemp * 1.8 + 32.0;
    ftemp
}

fn main() {
    println!("Please input the temperature: ");
    let mut temp_num = String::new();

    io::stdin()
        .read_line(&mut temp_num)
        .expect("Failed to read line");

    let temp_num = match temp_num.trim().parse() {
        Ok(num) => num,
        Err(_) => println!("Not a valid number?"),
    };


    let temp_num = { fahrenheit_to_celsius(temp_num) };
    let temp_num = { temp_num.to_string() };

    println!("{}", temp_num);
}
