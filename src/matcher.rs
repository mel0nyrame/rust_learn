// 模式匹配 2026/07/13

pub mod matcher {
    
    pub fn simple_match_example() {
        let c = 2;
        match c {
            1 => {
                println!("c is {}", 1)
            }

            _other => {
                println!("c is not 1")
            }
        }
    }
    
}