pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod network {
    fn connect() {
    }
}
mod clinet{
    fn connect(){
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
