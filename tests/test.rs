#[cfg(test)]
mod test {
    #[test]
    fn hello() {
        let hello = "Hello, world!";
        println!("Hello, world!");

        assert_eq!("Hello, world!", hello)
    }
}
