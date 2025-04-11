fn main() {
    // Example of how to define and use a function in Rust

    fn greet(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    let user_name = "Alice";
    println!("{}", greet(user_name));
}
