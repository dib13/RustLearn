fn main() {
    pattern_matching();
}

fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges!", x, how_many(x));
    }

    let point = (3, 4);

    match point // use ref in matching to address a reference.
    {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}", y),
        (x,0) => println!("x = {}, y axis", x),
        (x,y) => println!("x = {}, y = {}", x, y),

    }
}

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9...11 => "lots of",
        _ if (x % 2 == 0) => "some",
        _ => "a few",
    }
}
