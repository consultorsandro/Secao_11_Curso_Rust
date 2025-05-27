#[derive(Debug)] // Class 185

struct DeliSandwich {}

fn main() {
    println!("{}", identity(5));
    println!("{}", identity(13.94));
    println!("{}", identity("Hello, Rust!"));
    println!("{}", identity(String::from("Hello, Rust!")));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwich {}));
}



fn identity<T>(value: T) -> T {
    value
}