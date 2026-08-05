fn main() {
    // Vector
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(8);
    v.push(7);
    println!("{:?}", v);
    let v2 = vec![1,2,3,4];
    println!("{:?}", v2);

    // let third_in_v: &i3 = &v[9]; // break the program
    let third2_in_v: Option<&i32> = v.get(9); // return none
    
    for i in &v {
        println!("{i}");
    }

}
