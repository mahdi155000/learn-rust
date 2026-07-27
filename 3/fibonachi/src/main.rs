fn main() {
    let mut a = 0;
    let mut b = 1;

    for i in 1..10{
        let next = a + b;
        a = b;
        b = next;
        println!("{}", b);
    }
}
