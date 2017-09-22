// This is a program to convert Fahrenheit to Celsius and vice-versa.
use std::io;

fn fahrenheit_to_celsius(ftemp: u32) -> f64 {
    let work = ftemp as f64;
    let ctemp = (work - 32.0) / 1.8;
    ctemp
}

fn celsius_to_fahrenheit(ctemp: u32) -> f64 {
    let work = ctemp as f64;
    let ftemp = work * 1.8 + 32.0;
    ftemp
}

fn main() {
    println!("Please input the temperature: ");
    let mut temp_num = String::new();

    io::stdin()
        .read_line(&mut temp_num)
        .expect("Failed to read line");

    let temp_num : u32 = match temp_num.trim().parse() {
        Ok(num) => num,
        Err(_) => 100,
    };

    let tempc = fahrenheit_to_celsius(temp_num);
    let tempf = celsius_to_fahrenheit(temp_num);

    println!("Your temperature in F is: {}", tempf);
    println!("Your temperature in C is: {}", tempc);
}
