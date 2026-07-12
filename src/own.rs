// 所有权系统 2026/07/13

pub mod owner_system {
    pub fn simple_example() {
        // 所有权示例
        let s1 = String::from("123");
        let _s2 = s1;

        // println!("{}",s1); 报错,此时s1被移动到了s2,不同于浅拷贝和深拷贝

        let int1 = 32;
        let int2 = int1;

        // 基本数据类型都实现了Copy trait,这个trait可以让移动之后的变量依然能使用
        println!("int1:{} int2:{}", int1, int2);
    }
}

pub mod borrow {
    pub fn simple_borrow_example() {
        // 引用借用示例
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        println!("{}", r3);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
}