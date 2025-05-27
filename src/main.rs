enum Cheesesteak<T> {
    Plain,
    Topping(T),
}
fn main() { 
  
let mushroom =  Cheesesteak::Topping("Mushroom");
let onions = Cheesesteak::Topping("Onions".to_string());
let topping = "bacon".to_string();
let bacon = Cheesesteak::Topping(&topping);

let mut plain: Cheesesteak<String> = Cheesesteak::Plain;
plain = Cheesesteak::Topping("Sausse".to_string());

}
/*
#[derive(Debug)] // Class 188

struct TreasureChest<T> {
    captain: String,
    treasure: T,
}

impl TreasureChest<String> {
    fn clean_treasure(&mut self) {
        self.treasure = self.treasure.trim().to_string(); // Class 190
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treasure(&self) -> usize { // Class 190
        self.treasure.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {  // Class 190
        self.captain.to_uppercase()
    }
}


fn main() { 
    let gold_chest = TreasureChest {
        captain: String::from("Fierbeard"),
        treasure: "Gold",
    };
    println!("{:?}", gold_chest);

    let mut silver_chest = TreasureChest {
        captain: String::from("Silverhand"),
        treasure: String::from("     Silver     "),
    };
    silver_chest.clean_treasure(); // Class 190
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("Blackbeard"),
        treasure: ["Gold", "Silver", "Diamonds"],
    };
    println!("{}", special_chest.amount_of_treasure()); // Class 190
    println!("{}", special_chest.capital_captain()); // Class 190
    println!("{:?}", special_chest);
}
*/

/*
fn make_tuple<T, U>(first: T, second: U) -> (T, U) { // Class 187
    (first, second)
}
fn main() {
    make_tuple(5, "Hello"); // Class 187
    make_tuple(3.14, true);
*/
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
