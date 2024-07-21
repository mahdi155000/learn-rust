fn main() {
    println!("Hello, world!");


    test_function(16);

    let x = five();
    println!("The value of five is: '{}'", x);
    println!("The value of function five is: '{}'", five());
}
// this is my first comment
fn test_function(x: i16)
{
    println!("The value of x is: '{}'", x);
}

fn five() -> i16
{
    5
}


