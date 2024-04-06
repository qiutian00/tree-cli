#[cfg(test)]
mod test {
    #[test]
    fn hello() {
        let hello = "Hello, world!";
        println!("Hello, world!");

        assert_eq!("Hello, world!", hello)
    }

    #[test]
    fn goodbye() {
        let goodbye = "Goodbye, world!";
        println!("Goodbye, world!");

        assert_eq!("Goodbye, world!", goodbye)
    }

    // test directory
    // ├── test1
    // │   ├── test1-1
    // │   └── test1-2
    // └── test2
    //     ├── test2-1
    //     └── test2-2
    #[test]
    fn test_dir() {
        let test_dir = "test directory";
        println!("test directory");

        assert_eq!("test directory", test_dir)
    }

}
