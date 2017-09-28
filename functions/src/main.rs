fn main() {
    functions();
    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn functions() {
    print_value(33);
    let mut z = 1;
    increase(&mut z);
    println!("z= {}", z);

    let a = 3;
    let b = 5;
    let p = product(a, b);
    println!("{} times {} is {}.", a, b, p);
}

fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x += 1; // * means to dereference and change original value.
}

fn product(x: i32, y: i32) -> i32 {
    x * y
}
