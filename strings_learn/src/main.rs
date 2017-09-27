fn main() {
    strings();
}

fn strings() {
    // utf-8
    let s = "hello there!"; //&'static str is a vector of chars. &str is a string slice.
                            // s = "abc"; won't work.

    for c in s.chars().rev()
    //chars is a special function to give you a sequence of chars. rev Reverses.
    {
        println!("{}", c);
    }

    if let Some(first_char) = s.chars().nth(0) {
        println!("first letter is {}", first_char);
    }

    //String
    let mut letters = String::new();
    let mut a = 'a' as u8; // u8 will convert it to it's bytecode value.
    while a <= ('z' as u8) {
        letters.push(a as char); //Because String is also a vector, we can push onto it.
        letters.push_str(","); // To push a str slice to it, use push_str.
        a += 1;
    }

    println!("{}", letters);

    // String -> &str
    //let u:&str = &letters;

    //Concatenation
    // let z = letters + &letters; Adding a reference makes a String an &str. (&str exist on the heap).

    //Making a String from a &str:
    let mut abc = String::from("hello world"); // or "hello world".to_string()
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));


}
