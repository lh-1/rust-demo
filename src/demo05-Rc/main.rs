use std::rc::Rc;
use std::thread;
use std::time::Duration;

/**
    RC 单线程使用，且只读
*/

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Rc::new(Point { x: 25, y: 30 });

    let p1 = Rc::clone(&p);
    let p2 = Rc::clone(&p);

    println!("{} {} {}", p.x, p1.x, p2.x);
}