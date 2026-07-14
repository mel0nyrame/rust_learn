pub mod vec {

    #[test]
    pub fn simple_vec_example() {

        // 创建基础vec
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

        // 使用宏创建vec
        let vec2 = vec![1,9,1,9,1,8,0];

        // 这里是 Option<&T>
        let index2 = vec2.get(2);

        match index2 {
            Some(1) => println!("{:?},{}", index2,index2.unwrap()),
            // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
            None => println!("None"),
            _ => {}
        }

        for i in &vec2 {
            println!("{}",i)
        }

        for i in 0..vec2.len() {
            vec1.push(vec2[i]);
        }

        println!("{:?}", vec1);
    }

    // Abhorrent Java mindset
    pub mod object_list_example {
        #[derive(Debug)]
        pub enum CsvRow {
            DATE(i64),
            REGION(String),
            VALUE(String),
        }

        #[test]
        pub fn object_list() {
            let list: Vec<CsvRow> = vec![
                CsvRow::DATE(20260714),
                CsvRow::REGION("CH".to_owned()),
                CsvRow::VALUE("IDK".to_owned()),
            ];

            println!("{:?}", list);
        }

    }
}

pub mod string {

    #[test]
    pub fn simple_str_example() {
        let s1 = "hello world";
        let s2 = String::from("hello world");

        println!("{}", s1.eq(&s2));
        println!("{}",s1 == s2);

        let mut s3 = String::from("hello");
        s3.push_str(" world");
        println!("{}", s3);

        let hello = "Здравствуйте";

        let s4 = &hello[0..4];
        println!("{}",s4); // 不是Здра

        for c in hello.chars() {
            println!("{}",c);
        }

        for b in hello.bytes() {
            println!("{}",b);
        }
    }
}