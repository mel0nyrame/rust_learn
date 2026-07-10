use std::f64::consts::PI;
use rand::RngExt;

const M: u64 = 1000003;

fn main() {
    // 元组
    let simple_tuple: (i32, i32, i32) = (1, 2, 3);
    println!("{}", simple_tuple.1);
    let (a, b, c) = simple_tuple;
    println!("{},{},{}", a, b, c);

    // 数组
    let simple_array = [1, 2, 3, 4, 5];
    println!("{:?} {}", simple_array, simple_array[1]); // [1, 2, 3, 4, 5] ?
    let simple_array1 = [true; 5];
    println!("{:?}", simple_array1);

    // 条件语句
    const VALUE: i8 = 0;
    let value: i32 = if VALUE == 0 { 114514 } else { 1919180 };
    println!("{}", value);

    // 小的布尔过滤器
    let mut bits = [false; 1000003];
    insert("hello", &mut bits);
    println!("{}", may_contain("hello", &bits));
    println!("{}", may_contain("world", &bits));

    // 所有权示例
    let s1 = String::from("123");
    let s2 = s1;

    // println!("{}",s1); 报错,此时s1被移动到了s2,不同于浅拷贝和深拷贝

    let int1 = 32;
    let int2 = int1;

    // 基本数据类型都实现了Copy trait,这个trait可以让移动之后的变量依然能使用
    println!("int1:{} int2:{}", int1, int2);

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

    // 枚举
    let var = Type::VAR;
    println!("{:?}", var);

    let x: i8 = 5;
    let y: Option<i8> = Some(x);

    // let sum = x + y; 报错因为不同类型

    let c = 2;
    match c {
        1 => {
            println!("c is {}", 1)
        }

        other => {
            println!("c is not 1")
        }
    }
}
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
    fn compare_to_age(&self, user: &User) -> bool {
        self.age > user.age
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn hash_with_seed(s: &str, seed: i32) -> i32 {
    let mut h = seed as u64;

    for &byte in s.as_bytes() {
        let c = (byte as i8) as u64;
        h = h.wrapping_mul(131).wrapping_add(c);
    }

    (h % M) as i32
}

fn insert(s: &str, arr: &mut [bool]) {
    for seed in [31, 131, 1313] {
        let index = hash_with_seed(s, seed) as usize;
        arr[index] = true;
    }
}

fn may_contain(s: &str, arr: &[bool]) -> bool {
    for seed in [31, 131, 1313] {
        let index = hash_with_seed(s, seed) as usize;
        if !arr[index] {
            return false;
        }
    }
    true
}

// 摄氏度转华氏度
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

// 华氏度转摄氏度
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    fahrenheit - 32.0 * 5.0 / 9.0
}

// 生成第n个斐波那契数
fn fibonacci(n: i32) -> i64 {
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