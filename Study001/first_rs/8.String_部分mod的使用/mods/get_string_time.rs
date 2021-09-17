use std::{thread, time::SystemTime, time};
pub fn work_time() {
    println!("比较Rust三种定义字符串的方法：");
    println!("1.计算to_string时间：");
    let sys_time0 = SystemTime::now();
    let n = 10000; // 1万次
    for _ in 0..n{
        let _x:String = "hello".to_string();
    }
    println!("to_string()=>time:{}second", SystemTime::now().duration_since(sys_time0).unwrap().as_secs());

    println!("2.计算String::from时间：");
    let sys_time1 = SystemTime::now();
    for _ in 0..n{
        let _x:String = String::from("hello");
    }
    println!("String::from()=>time:{}second", SystemTime::now().duration_since(sys_time1).unwrap().as_secs());

    println!("3.计算into()时间：");
    let sys_time2 = SystemTime::now();
    for _ in 0..n{
        let _x:String = "hello".into();
    }
    println!("into()=>time:{}second", SystemTime::now().duration_since(sys_time2).unwrap().as_secs());

    thread::sleep(time::Duration::from_millis(2000));
}