fn main() {
    let rectangle = (30, 50);

    println!("The area of the rectangle is {} sqaure pixels.",
            area(rectangle));
   
}


fn area(domainsions: (u32, u32)) -> u32 {
    domainsions.0 * domainsions.1
}

fn build_user(email:String, username: String) -> User{
    User{
        email,
        username,
        active: true,
        sign_in_count: 1,

    }
}


struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}
