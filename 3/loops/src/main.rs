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


    let tuple_a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: '{}'", tuple_a[index]);
        index += 1;
    }
}
