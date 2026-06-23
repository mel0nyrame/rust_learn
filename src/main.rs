const M:u64 = 1000003;

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
    println!("{}",value);


    // 小的布尔过滤器
    let mut bits = [false; 1000003];
    insert("hello", &mut bits);
    println!("{}", may_contain("hello", &bits));
    println!("{}", may_contain("world", &bits));
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
