fn main() 
{
    let _n = 100;
    let mut first = 0;
    let mut second = 1;


//    println!("'{}'\n'{}'\n'{}'", first, second, second);
    for i in (2.._n)
    {
        println!("'{}', ",first);
        let third = first + second;

        first = second;
        second = third;
    }

    


}


