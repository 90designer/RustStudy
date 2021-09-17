
// ====[rust HashMap<K,V>]=====
// 键值对
// 同样存在所有权问题

use std::collections::HashMap;

use first_rs::hash_map;
fn main() {
    // 创建
   let (mut hashM1, hashM2) = hash_map::create::create();
    
    // 获取
   hash_map::get::get(&hashM1, String::from("b"));

   // 遍历
   hash_map::foreach::foreach(&hashM1);

    // 修改
    hash_map::update::update(&mut hashM1, "c", 20);
    hash_map::get::get(&hashM1, String::from("c"));


    let text = "this is a test for hash map test";
    let mut map = HashMap::new();
    let mut val = 100;
    for word in text.split_whitespace(){
        if word == "test"{
           let count = map.insert(word, val);
           val += 100;
        }else{
            let count = map.entry(word).or_insert(0);
            *count += 999;
        }
        
    }

    println!("{:#?}",map);
}