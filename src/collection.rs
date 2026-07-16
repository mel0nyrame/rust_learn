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
        let vec2 = vec![1, 9, 1, 9, 1, 8, 0];

        // 这里是 Option<&T>
        let index2 = vec2.get(2);

        match index2 {
            Some(1) => println!("{:?},{}", index2, index2.unwrap()),
            // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None
            None => println!("None"),
            _ => {}
        }

        for i in &vec2 {
            println!("{}", i)
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
        println!("{}", s1 == s2);

        let mut s3 = String::from("hello");
        s3.push_str(" world");
        println!("{}", s3);

        let hello = "Здравствуйте";

        let s4 = &hello[0..4];
        println!("{}", s4); // 不是Здра

        for c in hello.chars() {
            println!("{}", c);
        }

        for b in hello.bytes() {
            println!("{}", b);
        }
    }
}

pub mod map {
    #[test]
    pub fn simple_hashmap_example() {
        use std::collections::HashMap;

        let mut map1: HashMap<String, i32> = HashMap::new();

        let map1_is_empty = map1.is_empty();
        println!("{}", map1_is_empty);

        map1.insert("zhangsan".to_string(), 32);
        map1.insert("lisi".to_string(), 64);

        println!("{:?}", &map1);

        match map1.get("zhangsan") {
            Some(t) => println!("zhangsan's value is {}", t),
            None => println!("zhangsan is None"),
        }

        let value = map1.entry("zhangsan".to_string()).or_insert(1);

        println!("{}", value)
    }

    pub mod pratice {
        #[test]
        pub fn get_vec_average_median_and_mode() {
            use std::collections::HashMap;

            let mut arr: Vec<i32> = vec![73, 38, 17, 44, 95, 47, 68, 90, 56, 15];

            let mut sum = 0;
            for num in &arr {
                sum = sum + num;
            }

            println!("arr's average is {}", sum / arr.len() as i32);

            arr.sort();
            println!("arr's median is {}", arr[arr.len() / 2 - 1]);

            let mut map: HashMap<i32, i32> = HashMap::new();
            for num in &arr {
                let cnt = map.entry(*num).or_insert(0);
                *cnt += 1;
            }

            let max_count = map.values().max().unwrap_or(&0);
            let modes: Vec<i32> = map
                .iter()
                .filter_map(|(&num, &count)| if count == *max_count { Some(num) } else { None })
                .collect();

            println!("arr's mode(s): {:?}", modes);
        }

        #[test]
        pub fn string_to_pig_latin() {
            let text = "The Complete Works of William Shakespeare
        Welcome to the Web's first edition of
        the Complete Works of William
        Shakespeare";

            let mut pig_latin_words = Vec::new();

            for word in text.split_whitespace() {
                if word.is_empty() {
                    continue;
                }

                let first_char = word.chars().next().unwrap();
                let is_vowel =
                    matches!(first_char.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u');

                let pig_word = if is_vowel {
                    format!("{}-way", word)
                } else {
                    let rest: String = word.chars().skip(1).collect();
                    format!("{}-{}ay", rest, first_char)
                };

                pig_latin_words.push(pig_word);
            }

            let result = pig_latin_words.join(" ");
            println!("{}", result);
        }

        pub mod practice3 {
            use std::collections::HashMap;

            pub fn push_name_to_department(cmd: &str, map: &mut HashMap<String, Vec<String>>) {
                let words: Vec<&str> = cmd.split_whitespace().collect();
                if words.len() < 4 {
                    return;
                }

                let first = words[0].to_lowercase();
                let third = words[2].to_lowercase();
                if first != "add" || third != "to" {
                    return;
                }

                let name = words[1].to_string();
                let dept = words[3..].join(" ");
                if dept.is_empty() {
                    return;
                }

                map.entry(dept).or_insert_with(Vec::new).push(name);
            }

            pub fn get_names_from_department(
                department: &str,
                map: &HashMap<String, Vec<String>>,
            ) -> Vec<String> {
                match map.get(department) {
                    Some(employees) => {
                        let mut sorted = employees.clone();
                        sorted.sort();
                        sorted
                    }
                    None => Vec::new(),
                }
            }
        }
    }
}
