fn main() {
    generics();
}

struct Point<T> {
    x: T,
    y: T,
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}
fn generics() {
    let a: Point<f64> = Point { x: 0f64, y: 4f64 }; //Explicit Point<i32,u16> for example.
    let b = Point { x: 1.2, y: 3.4 };

    let my_line = Line{start: a, end: b};
}
