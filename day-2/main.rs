use std::io;

fn main() {
    println!("Hello to My 30-days Rust programming language!");
    let mut name = String::new();
    println!("Please input your name.");

    io::stdin().read_line(&mut name);

    let name = name.trim();

    println!("Hello, {}", name);
}