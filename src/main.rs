use area_gen::generate_area;
use std::thread;

mod area_gen;

fn main() {
    let mut handlers = Vec::new();

    // roll each maps in a sperate thread
    for i in 0..1 {
        handlers.push(thread::spawn(move || {
            generate_area(i);
        }));
    }
    for handler in handlers {
        handler.join().unwrap();
    }
}
