use std::io;

fn main() {
    let mut temprature = String::new();

    println!("Please enter a Farenheit number: ");
    io::stdin().read_line(&mut temprature)
        .expect("Failed to read line");


    let _temp: f32 = match temprature.trim().parse(){
        Ok(v) => v,
        Err(_) => {
            println!("That wasn't a valid integer!");
            return;
        }
    };

    let _temp = (_temp - 32.0) * 5.0  / 9.0;
    println!("The temprature in Celciuse is: {}", _temp);


    
}
