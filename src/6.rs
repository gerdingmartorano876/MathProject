  use std::{thread,time};

fn main() {
    let mut numbers = (1..10).collect::<Vec<i32>>();
    thread::sleep(time::Duration::from_millis(5));
    numbers.sort();
}