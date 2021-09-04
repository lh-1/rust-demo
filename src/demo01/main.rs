struct Point {
    x: i32,
    y: i32,
}

fn simple_ownership() {
    let p1 = Point{x:25, y: 25};
    let p2 = p1;

    print!("{} {}", p2.x, p2.y);
}

fn main() {
    println!("Hello, world!");

    simple_ownership();
}
