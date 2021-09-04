use std::time::Duration;
use std::thread;

fn main() {

    let h = thread::spawn(||{
        for i in 0..10 {
            println!("{}", i);

            thread::sleep(Duration::from_millis(500));
        }
    });

    h.join();

}
