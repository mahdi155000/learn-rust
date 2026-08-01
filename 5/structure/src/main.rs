// #[derive (Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}


fn main() {
    // let rectangle = (30, 50);
    let rect1 = Rectangle{length: 50, width: 30};
    let rect2 = Rectangle{length: 40, width: 10};
    let rect3 = Rectangle{length: 45, width: 60};
    let rect4 = Rectangle{length: 50, width: 10};

    // println!("The area of the rectangle is {} sqaure pixels.",
    //         area(&rect1));

    // println!("the full structure is {:?}", rect1);

    println!("The area of rect1 is: {}", rect1.area());
    println!("can rect 1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect 1 hold rect2? {}", rect1.can_hold(&rect3));
    println!("can rect 1 hold rect2? {}", rect1.can_hold(&rect4));
   
}


// fn area(domainsions: (u32, u32)) -> u32 {
//     domainsions.0 * domainsions.1
// }

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.width * rectangle.height
// }

// fn build_user(email:String, username: String) -> User{
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,

//     }
// }


// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u32,
//     active: bool,
// }
