fn main()
{
    let x: u16 = "623".parse().expect("Not a number!");
    let binary = 0b1000_1000;
    let decimal = 5_1234;
    let hex = 0xfa;
    
    println!("The value of x is: '{}'", x);
    println!("The value of binary is: '{}'", binary);
    println!("The value of decimal is: '{}'", decimal);
    println!("The value of hex is: '{}'", hex);

}
