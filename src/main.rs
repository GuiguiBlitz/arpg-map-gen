use area_gen::generate_area;
use std::thread;

mod area_gen;

fn main() {
    // roll each maps in a sperate thread
    // for _ in 0..100 {
    let mut handlers = Vec::new();
    for i in 0..5 {
        handlers.push(thread::spawn(move || {
            generate_area(i);
        }));
    }
    for handler in handlers {
        handler.join().unwrap();
    }
    // }
}
