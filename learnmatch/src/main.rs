fn main() {
    match_statement();
}

fn match_statement()
{
    let country_code = 999; // 1-999

    let country = match country_code 
    {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1...999 => "Unknown", // Cannot use .. in a match statement. ... is last number inclusive.
        _ => "Invalid"
    };

    println!("The country with code {} is {}.", country_code, country);
}
