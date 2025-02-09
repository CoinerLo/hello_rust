use std::{thread, time::Duration};

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let handle = thread::spawn(move || {
        for i in v {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
