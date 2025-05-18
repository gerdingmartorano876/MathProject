use std::collections::VecDeque;

fn main() {
    let numbers: VecDeque<f64> = vec![-2.0, 1.5, 3.5, -1.0];
    
    for &number in &numbers {
        println!("{}", number.abs());
    }
}
