pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
use a::series::of;
// use TrafficLight::{Red, Yellow};
use TrafficLight::*;
fn main() {
//  a::series::of::nested_modules();
    of::nested_modules();
    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;
    println!("hello world");
    
}
