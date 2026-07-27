fn main(){
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    let z = five();
    println!("The value of z is: {}", z);

    let condition = true;
    let numbers = if condition {
        5
    } else {
        6
    };

    println!("The value of numbers is: {}", numbers);
}



fn five() -> i32 {
    5
}