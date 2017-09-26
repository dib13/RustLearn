use std::mem;

fn arrays()
{
    let mut a = [1,2,3,4,5]; //[type;how many] (you can also just leave it out if the elements are the same.)

    println!("a has {} elements, first is {}", a.len(), a[0]); //len is length of array.

    a[0] = 321;
    println!("a[0] = {}", a[0]);
    println!("{:?}", a); // {:?} is debug.

    if a != [1,2,3,4,5]{
        println!("Does not match!");
    }

    let b = [1u16; 10];
    for i in 0..b.len()
    {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));
    
    let mtx:[[f32;3];2] = 
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    // Get the diagonal values of the multi-level array?

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

fn main() {
    arrays();
}
