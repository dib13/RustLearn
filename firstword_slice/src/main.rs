fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let s = String::from("Hello World!");
    println!("{:?}", s);
    let word = first_word(&s);
    println!("{:?}", word)
    //s.clear();
}
