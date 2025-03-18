// This is an example of a Rust function that takes in two numbers and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is an example of a Rust function that takes in three numbers and returns the largest of them
fn max3(a: i32, b: i32, c: i32) -> i32 {
    if a >= b && a >= c {
        return a;
    } else if b >= a && b >= c {
        return b;
    } else {
        return c;
    }
}
