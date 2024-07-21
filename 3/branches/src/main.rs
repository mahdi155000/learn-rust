fn main() {
    println!("Hello, world!");

    let number = 6;
    //The condition is this code must be 'bool' otherwise we'll get an error.
    if number != 5
    {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }

    if number % 4 == 0
    {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }
}

