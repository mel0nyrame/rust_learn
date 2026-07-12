pub mod vec {

    #[test]
    pub fn simple_vec_example() {

        let mut vec1: Vec<i32> = Vec::new();
        let cap = vec1.capacity();
        println!("{}", cap);

        vec1.push(1);
        vec1.push(1);
        vec1.push(4);
        vec1.push(5);
        vec1.push(1);
        vec1.push(4);

        println!("{:?}", vec1);
    }
}