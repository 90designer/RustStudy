// use std::io; 

// use rand::Rng; // trait
// fn main() {
//     println!("====猜数游戏!====");
//     let secret_number = rand::thread_rng().gen_range(10..101);
//     println!("猜测一个数!{}",secret_number);

//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("无法读取");
//     println!("你猜测的数字是:{}" , guess);

// }
// use std::io;

// fn main(){
//     let mut input = String::new();
//     println!("hello");
//     match io::stdin().read_line(&mut input){
//         Ok(_) => {
//             println!("say: {}", input);
//         }
//         _ => {}
//     }
// }

fn main(){
    // Rust变量和可变性
    let a  = 5;
    let b = 5.01;
    let c = true;
    let d = 'a';
    let e = '\u{1f601}';
    let f  = "string"; //  不可变的字符串
    let mut g = "mut string";  // 可变变量
    g = "哈哈哈可变的";
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
    println!("{}", g);

}