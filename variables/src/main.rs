use std::mem;

const MEANING_OF_LIFE: u8 = 42; // no fixed address.

static mut Z: i32 = 123;

fn operators() {
    // arithmetic
    let mut a = 2 + 3 * 4; //BEDMAS
    a = a + 1; // NO -- or ++.
    a -= 2; // -= += classic.
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); //Integral power.
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    //bitwise
    let c = 1 | 2; // | is OR, & is AND, ^ is XOR, ! is NOR.
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    //logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
                                                // > < <= >= == (Equality)
    let x = 5;
    let x_is_5 = x == 5; //true
}

fn variables() {
    //unsigned 0+ (u)
    // signed -0+ (i)
    let a: u8 = 123; // 8bits
    println!("a = {}", a);

    //a = 456;

    //mut
    let mut b: i8 = 0; //mutable
    println!("b = {}", b);
    b = 42;
    println!("b = {}", b);

    let mut c = 123456789; // 32-bit signed
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!(
        "c = {} after modification, size = {} bytes",
        c,
        mem::size_of_val(&c)
    );
    // i8 u8 i16 u16 i32 u32 i64 u64

    let z: isize = 123; // isize/usize gives size of standrd memory address.
    let size_of_z = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit os",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    //true false

    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&e));
    //let m = true;
}

fn scope_and_shadowing() {
    let a = 123;
    println!("a = {}", a);
    {
        let b = 456;
        println!("inside, b = {}", b); // b doesn't exist outside these curly brackets.

        let a = 777;
        println!("inside, a = {}", a);
    }
}

fn main() {
    //scope_and_shadowing();
    unsafe {
        println!("{}", Z);
    }
}
