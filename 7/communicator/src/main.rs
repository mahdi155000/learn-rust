pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}
use a::series::of;
fn main() {
//  a::series::of::nested_modules();
    of::nested_modules();
    
}
