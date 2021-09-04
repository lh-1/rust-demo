use std::thread;
use std::time::Duration;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    new_thread();
}

fn new_thread(){
    let p1 = Point { x: 25, y: 30 };

    let h = thread::spawn(move || {
        println!("{} {}", p1.x, p1.y);
    });

    h.join();
}