fn main(){
    loop {
        println!("again");
        break;
    }

    let mut number = 3;
     while number != 0 {
        println!("{}!", number);

        number = number -1;
     }

     let a = [10, 20, 30, 40, 50];
     let mut index = 0;

     while index < 5 {
        println!("The value is: {}", a[index]);
        index = index + 1;
     }
     println!("-------------------------------------");

     for item in a.iter() {
        println!("{}", item);
     }
}
