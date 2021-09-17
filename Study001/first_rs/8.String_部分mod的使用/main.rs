
// ====[rust String]===
use first_rs::mods;
fn main() {
    // let a = String::new();
    // let b = "转换".to_string();
    // let c= String::from("传递");
    // let d:String = "hello".into();

    mods::get_string_time::work_time();

    let mut s = String::from("Abc");
    s.push_str("d");
    s.push('e');

    // +号连接字符串，前面参数移动，后面是引用
    // let b = String::from("测试链接");
    // let sb =  s + &b;
    // println!("{}",s);  

    let s1 = String::from("AAA");
    let s2 = String::from("BBB");
    let s3 = String::from("CCC");
    let s4 = String::from("ddd");

    // 因为 字面量"_"本身就是&str字符串切片，所以+表达式成立。
    let s10 = s1 + "_" + &s2 +"_" + &s3;
    println!("s10={}",s10);
    let s11 = format!("{}-{}-{}", s4, s2, s3);
    println!("s11={}",s11);

    // 字节
    let china = "中国";
    for i in china.bytes(){
        println!("{}",i);
    }

    // unicode标量值
    for i in china.chars(){
        println!("{}",i);
    }

    // 字形簇
    // 貌似暂不支持


    let c1 = &china[0..3];
    println!("{}",c1);
}
