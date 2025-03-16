
use std::{thread,time};
fn main() {
let duration = time::Duration::from_secs(5);
thread::sleep(duration);
println!("Hello World");
}