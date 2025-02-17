#[derive(Debug)]
enum UsState {
    Albama,
    Alaska,
    NewYork,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter (UsState),
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1,
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        },
	}
}
fn main() {

    println!("Hello, world!");
}
