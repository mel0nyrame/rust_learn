// 结构体和枚举 2026/07/13
#[allow(dead_code)]
pub mod structs {
    #[derive(Debug)]
    enum Type {
        VAR,
        CONST,
    }

    #[derive(Debug)]
    struct User {
        name: String,
        age: i32,
        sex: String,
    }

    impl User {

        fn compare_to_name(&self, user: &User) -> i8 {
            for i in self.name.chars() {
                for j in user.name.chars() {
                    if i > j {
                        return 1;
                    }
                }
            }
            0
        }

        fn compare_to_age(&self, user: &User) -> bool {
            self.age > user.age
        }

        fn print_sex(&self) {
            println!("{}",self.sex)
        }
    }

    pub fn simple_struct_example() {
        // 结构体
        let user = User {
            name: String::from("zhangsan"),
            age: 12,
            sex: String::from("man"),
        };
        println!("{:?}", user);

        let user2 = User {
            name: String::from("lisi"),
            age: 11,
            sex: String::from("man"),
        };

        let result = User::compare_to_age(&user, &user2); // 实际上类似于java中的静态方法

        println!("{}", result);
    }

    pub fn simple_enum_example() {
        // 枚举
        let var = Type::VAR;
        let con = Type::CONST;
        println!("{:?},{:?}", var,con);

        let x: i8 = 5;
        let _y: Option<i8> = Some(x);

        // let sum = x + y; 报错因为不同类型
    }
}