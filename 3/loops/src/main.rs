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
}
