fn main() {
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1); // with specific data type
    let tup1 = (500, 4.2, 5) ;
    let (_x, _y, _z) = tup1 ;

    println!("The value of z is: '{}'", _z);


    let t = ( 500, 2.3, 'z');
    let five_hundred = t.0;
    let two_point_three = t.1;
    let char_z = t.2;

    // Array

    let a = [1,2,4,5,6];

    let fist = a[0];

    println!("first in array is: '{}'", fist);

}
