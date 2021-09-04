use std::thread;
use std::time::Duration;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Box::new(Point { x: 25, y: 30 });

    println!("{}", (*p).x);// 标准写法
    println!("{}", (&p).x);
    println!("{}", p.y);
}