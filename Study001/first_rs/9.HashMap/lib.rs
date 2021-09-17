pub mod hash_map{
    pub mod create{
        use std::collections::HashMap;
        pub fn create()->(HashMap<String, i32>, HashMap<i32,i8>){
            let mut scores = HashMap::new();
            scores.insert(String::from("a"), 100);
            scores.insert(String::from("b"), 200);

            let mut scores2:HashMap<i32, i8> = HashMap::new();
            (scores, scores2)
        }
    }

    pub mod get{
        use std::collections::HashMap;
        pub fn get(hash_map: &HashMap<String,i32>, key :String){
            match hash_map.get(&key){
                Some(val) => {
                    println!("获取Key:{} => val: {} ",key, val);
                },
                None => println!("没有值")
            }
        }
    }

    pub mod foreach {
        use std::collections::HashMap;
        pub fn foreach(hash_map: &HashMap<String,i32>){
            for (k,v) in hash_map {
                println!("Key:{}=>Val:{}",k,v);
            }
        }
    }
    pub mod update {
        use std::collections::HashMap;
        pub fn update(hash_map: &mut HashMap<String,i32>, key: &str, val:i32){
            hash_map.insert(String::from(key), val);
            println!("设置key：{}, val: {} 成功", key,val);

            hash_map.insert(String::from(key), val+val);
            println!("设置key：{}, val: {} 成功", key,val+val);

            let e = hash_map.entry(String::from(key)).or_insert(val);
            println!("{:#?}", e);

            let e = hash_map.entry(String::from("AA"));
            println!("{:#?}", e);
        }
    }
}