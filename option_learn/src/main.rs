fn main() {
    option();
}

fn option() {
    //Option<T>
    let x = 1.0;
    let y = 2.0;
    // Next statment can return either a Some(z) or None.
    let result: Option<f64> = if x != 0.0 && y != 0.0 {
        Some(x / y)
    } else {
        None
    };


    println!("{:?}", result);

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("Cannot divide {} by {}!", x, y),
    }
    // if let /while let to find out if you have a provided value.

    if let Some(z) = result {
        println!("z = {}", z);
    }
}
