use std::vec;

// =====【rust 集合】=====
// 1。 Vector
/**
 *  Vec<T>
 *  有标准库提供
 *  可存储多个值
 *  只能存储 相同（任意）类型的数据
 *  值在内存中连续存放
 * 
 *  同样不能同时存在 mut可变 和 &vec 不可变引用
 */
fn main(){
    // 定义
    let mut v :Vec<i32> = Vec::new();
    v = vec![1,2,3];
    v.push(5);

    let c = vec!['a','b','c'];

    let mut d = Vec::new();
    d.push("AA");

    // 读取
    let aa = &v[0];
    let bb = v.get(1);

    match v.get(10) {
        Some(attr) => println!("{}", attr),
        None => {
            println!("没有找到该索引对应的值,索引越界");
        }
    }
    
    for item in &mut v {
        *item += 1;
    }
    for i in v{
        print!("{},", i);
    }
   
    println!("");
    let mut desc:Vec<i32> = Vec::with_capacity(5);
    let first:Vec<i32> = vec![1,2,3,4];
    let last = vec![10,20,30,40];
    add_vec(&mut desc, &first, &last);
    for i in 0..first.len()  {
        print!("{},",desc[i]);
    }

    // 通过枚举
    let row= vec![
        num::Int(100),
        num::Float(10.22),
        num::Text(String::from("通过枚举创建Vector"))
    ];


}

fn add_vec(desc:&mut Vec<i32>, first:&Vec<i32>, last:&Vec<i32>){
    for i in 0..first.len() {
        desc.push( first[i] + last[i]*2);
      }
}
enum num{
   Int(i32),
   Float(f64),
   Text(String),
}