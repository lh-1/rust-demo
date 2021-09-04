use std::rc::Rc;
use std::sync::{Arc, Mutex};
// Atomic Rc
use std::thread;
use std::time::Duration;
use std::fmt::Debug;

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p_mutex = Mutex::new(Point { x: 25, y: 30 });

    let p_arc_mutex = Arc::new(p_mutex);

    for _ in 0..10 {
        let pp = Arc::clone(&p_arc_mutex);
        let h = thread::spawn(move || {
            let mut p = pp.lock().unwrap();
            p.x += 1;
        });

        h.join();
    }
    println!("{}", p_arc_mutex.lock().unwrap().x);

}