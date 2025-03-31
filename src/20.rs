use std::collections::VecDeque;

fn main() {
    let mut numbers: VecDeque<i32> = VecDeque::new();

    while !numbers.is_empty() {
        let num = numbers.pop_front().unwrap();
        println!("{}", num);
        if num % 10 == 9 || (num - 5).abs() < 5 && (num - 7).abs() > 3 {
            numbers.push_back(num);
        }
    }
}
