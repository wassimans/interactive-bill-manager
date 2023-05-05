use std::io;

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again")
    }
    let input = buffer.trim().to_owned();
    if input.is_empty() {
        None
    } else {
        Some(input)
    }
}
fn main() {
    println!("Hello, world!");
}
