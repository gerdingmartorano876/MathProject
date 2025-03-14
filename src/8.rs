use std::thread;
use std::time::Duration;

fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    thread::sleep(Duration::from_secs(2));
    println!("{:?}", numbers);
}
