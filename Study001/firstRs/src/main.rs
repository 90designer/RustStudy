use std::io; 
use std::cmp::Ordering;
use rand::Rng; // trait
fn main() {
    println!("====猜数游戏!====");
    let secret_number = rand::thread_rng().gen_range(10..101);
    println!("猜测一个数!{}",secret_number);

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取");
    println!("你猜测的数字是:{}" , guess);

    match guess.cmp(&secret_number){
        Ordering::Less=> println!("Too small!");
        Ordering::Greater => println!("Too big");
        Ordering::Equal => println!("You win")
    }
}
