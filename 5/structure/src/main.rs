fn main() {
   
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
    username: username,
    email: email,
    sign_in_count: u32,
    active: bool,
}
