fn main() {
    vectors();
}

fn vectors() {
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);

    let idx: usize = 1; //needs to be a usize.
    println!("a[0] = {}", a[idx]);

    match a.get(6) // Returns an option
    {
        Some(x) => println!("a[6] = {}", x),
        None => println!("error, no such element")
    }

    for x in &a {
        println!("{}", x);
    }

    let last_elem = a.pop(); //Option

    println!("Last elem is {:?}, a = {:?}", last_elem, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
