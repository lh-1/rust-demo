use std::rc::Rc;
use std::sync::Arc;
// Atomic Rc
use std::thread;
use std::time::Duration;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Arc::new(Point { x: 25, y: 30 });

    let p1 = Arc::clone(&p);
    let h1 = thread::spawn(move || {
        println!("{} {}", p1.x, p1.y);
    });

    let p2 = Arc::clone(&p);
    let h2 = thread::spawn(move || {
        println!("{} {}", p2.x, p2.y);
    });

    h1.join();
    h2.join();
}