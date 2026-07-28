fn main() {
    // let s = String::from("hello");

    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{s}");

    let str = String::from("hello"); // str come to scope
    take_owvership(str);                     // str's value move to the function
                                             // str is not valid here anymore
    let x = 5;
    makes_copy(x);




    let (s2, len) = calculate_length(s);





}

fn ownership () {
                                // s is not valid in this section
    let s = "Hello";      // s is valid now
                                // s is still valid
}                               // s is not valid anymore}

fn take_owvership (str: String){
    println!("string is {str}");

}

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len();
}