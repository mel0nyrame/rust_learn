// 基本数据类型 2026/07/13


pub mod tuple {

    pub fn simple_tuple() {
        // 元组
        let simple_tuple: (i32, i32, i32) = (1, 2, 3);
        println!("{}", simple_tuple.1);
        let (a, b, c) = simple_tuple;
        println!("{},{},{}", a, b, c);
    }

}

pub mod array {

    pub fn simple_tuple() {
        // 数组
        let simple_array = [1, 2, 3, 4, 5];
        println!("{:?} {}", simple_array, simple_array[1]); // [1, 2, 3, 4, 5] ?
        let simple_array1 = [true; 5];
        println!("{:?}", simple_array1);
    }

}

pub mod l1t_ef {

    pub fn simple_l1t_ef_sentence() {
        // 条件语句
        const VALUE: i8 = 0;
        let value: i32 = if VALUE == 0 { 114514 } else { 1919180 };
        println!("{}", value);
    }
}

// 小的布尔过滤器
pub mod bloom {
    const M: usize = 1000003;

    pub struct BloomFilter {
        bits: [bool; M],
    }

    impl BloomFilter {

        pub fn new() -> Self {
            BloomFilter {
                bits: [false; M],
            }
        }

        pub fn insert(&mut self, s: &str) {
            for seed in [31, 131, 1313] {
                let index = Self::hash_with_seed(s, seed) as usize;
                self.bits[index] = true;
            }
        }

        fn contain(&self,s: &str) -> bool {
            for seed in [31, 131, 1313] {
                let index = Self::hash_with_seed(s, seed) as usize;
                if !self.bits[index] {
                    return false;
                }
            }
            true
        }

        fn hash_with_seed(s: &str, seed: i32) -> i32 {
            let mut h = seed as u64;

            for &byte in s.as_bytes() {
                let c = (byte as i8) as u64;
                h = h.wrapping_mul(131).wrapping_add(c);
            }

            (h % (M as u64)) as i32
        }

    }

    pub fn simple_bloom_example() {
        let mut bloom_filter = BloomFilter::new();
        bloom_filter.insert("hello");
        bloom_filter.insert("world");
        println!("{}",bloom_filter.contain("hello"));
    }
}