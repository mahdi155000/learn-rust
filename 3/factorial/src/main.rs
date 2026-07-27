fn main() {
    let n = 10;

    // println!("1 * 1 = 1");
    let mut result = 1;
    for i in 1..=n {
        let x = result;
        result *= i;
        println!("{x} * {i} = {result}");

    }
}
