fn main()
{
    let x = 5;
    
    let x = x + 1;

    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "    ";
    println!("The value of spaces is: '{}'.END", spaces);
    let spaces = spaces.len();
    println!("There is: '{}' spaces .END", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The guess number is: {}", guess);

}
