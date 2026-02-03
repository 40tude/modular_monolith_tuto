// ex01.rs
// cargo run --example ex01
//
// Check if Roberto

fn main() {
    let greeting = greet("Roberto");
    println!("{}", greeting);
}

fn greet(name: &str) -> String {
    // Special case for Roberto
    if name == "Roberto" {
        return "Ciao Roberto!".to_string();
    }

    format!("Hello {}.", name)
}
