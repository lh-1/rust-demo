use std::rc::Rc;
use std::sync::Mutex;
// Atomic Rc
use std::thread;
use std::time::Duration;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p_mutex = Mutex::new(Point { x: 25, y: 30 });

    let mut p = p_mutex.lock().unwrap();

    p.x += 1;

    println!("{} {}", p.x, p.y);
}