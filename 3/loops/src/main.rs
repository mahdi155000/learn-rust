fn main() {
    // loop loop
    let mut loop_break = 0 ;
    loop {
        println!("again!");
        loop_break += 1;
        if loop_break >= 5 {
            println!("end of the loop.\n");
            break;
        }
    }

    let mut while_break = 10;

    while while_break != 0 {
        println!("{}!", while_break);
        while_break -= 1;
    }

    println!("LIFTOFF!!!\n\n");


    let array_a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("While loop result: ");
    while index < 5 {
        println!("The value is: '{}'", array_a[index]);
        index += 1;
    }

    // For loop
    println!("\nFor loop result: ");
    for item in array_a.iter()
    {
        println!("The value is: '{}'", item);
    }
}
