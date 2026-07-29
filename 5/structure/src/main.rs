fn main() {
    let mut user1 = User{
        email: String::from("mahdi155000@gmail.com"),
        username: String::from("mahdi155000"),
        active: true,
        sign_in_count: 0,
    };
    
}


struct User {
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}
