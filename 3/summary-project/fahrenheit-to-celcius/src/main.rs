fn main() {

    let fahrenheit: f32 = 64.0;
     
    let celsius: f32 = convert(fahrenheit);

    println!("the '{}' fahrenheit is: '{}' celsius",fahrenheit, celsius );


}


fn convert(fahrenheit: f32) -> f32
{
    (fahrenheit - 32.0) * 5.0 / 9.0
    //let mut _f: f32 = fahrenheit;

    //_f -= 32.0;
    //_f *= 5.0;
    //_f /= 9.0;
    //_f
}
