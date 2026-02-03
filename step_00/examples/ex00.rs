// ex00.rs
// cargo run --example ex00
//
// The very first try

fn main() {
    let greeting = greet("Bob");
    println!("{}", greeting);
}

fn greet(name: &str) -> String {
    format!("Hello {}.", name)
}
