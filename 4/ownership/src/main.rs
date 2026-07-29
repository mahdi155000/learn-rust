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

    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    println!("{word}")


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
    s.len()
}


fn fist_word2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        return &s[0..i];
    }

    &s[..]
    
}