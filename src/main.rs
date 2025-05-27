fn make_tuple<T, U>(first: T, second: U) -> (T, U) { // Class 187
    (first, second)
}
fn main() {
    make_tuple(5, "Hello");
    make_tuple(3.14, true);
}
/*
    println!("{}", identity::<i32>(5)); // Class 186
    println!("{}", identity::<f64>(13.94));
    println!("{}", identity::<&str>("Hello, Rust!"));
    println!("{}", identity::<String>(String::from("Hello, Rust!")));
    println!("{}", identity::<bool>(true));
    println!("{:?}", identity::<DeliSandwich>(DeliSandwich {}));
}

fn identity<T>(value: T) -> T {
    value
}
*/
/* Class 185
   println!("{}", identity(5));
    println!("{}", identity(13.94));
    println!("{}", identity("Hello, Rust!"));
    println!("{}", identity(String::from("Hello, Rust!")));
    println!("{}", identity(true));
    println!("{:?}", identity(DeliSandwich {}));
*/