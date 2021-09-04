use std::rc::Rc;
use std::sync::{Mutex, Arc};
// Atomic Rc
use std::thread;
use std::time::Duration;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p_mutex = Mutex::new(Point { x: 25, y: 30 });

    let p_arc_mutex = Arc::new(p_mutex);

    let p1 = Arc::clone(&p_arc_mutex);
    thread::spawn(move || {
        let mut p = p1.lock().unwrap();
        p.x += 1;

        println!("{} {}", p.x, p.y);
    }).join();

    let p2 = Arc::clone(&p_arc_mutex);
    thread::spawn(move || {
        let mut p = p2.lock().unwrap();
        p.x += 1;

        println!("{} {}", p.x, p.y);
    }).join();
}