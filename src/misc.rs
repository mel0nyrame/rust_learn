// 一些随便写的函数 2026/07/13

pub mod misc {
    use rand::RngExt;

    // 摄氏度转华氏度
    pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }

    // 华氏度转摄氏度
    pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
        fahrenheit - 32.0 * 5.0 / 9.0
    }

    // 生成第n个斐波那契数
    pub fn fibonacci(n: i32) -> i64 {
        if n <= 1 {
            n as i64
        } else {
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    }

    #[test]
    fn test_rand_mod() {
        // ? api改了大多数教程都过时了对于最新的rand
        // let random = rand::thread_rng().rng.gen_range(1..2);
        let random1 = rand::rng().random_range(1..2);
        let random2 = rand::random_range(1f64..2f64);
        println!("random1:{} random2:{}",random1,random2);
    }

    #[test]
    fn test_macro() {
        let x1: i32 = 2;
        let x2: i32 = 3;
        assert_eq!(x1, x2);
        println!("x1:{} x2:{}", x1, x2);
    }
}