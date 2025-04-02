use std::cmp;

fn main() {
    let numbers: Vec<i32> = vec![-5, 0, 1, 4, -2];
    let mut max = &numbers[0]; // Set max to the first element in the vector

    for &num in &numbers {
        if num > *max {
            *max = num;
        }
    }

    println!("The maximum number is: {}", max);
}
