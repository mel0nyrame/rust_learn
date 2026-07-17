pub mod panic {

    #[test]
    pub fn panic_example() {
        panic!("报错");
    }

    #[test]
    pub fn backtrace_example() {
        let v = vec![1, 2, 3];

        println!("{}", v[99]);
    }
}
