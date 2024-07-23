fn main()
{
    let _n = 10;

    let mut first = 0;
    let mut second = 1;

    let mut count = 0;
    while count < _n
    {
        count += 1;
        println!("'{}'", first);

        let third = first + second;

        first = second;
        second = third;

        
    }
}
