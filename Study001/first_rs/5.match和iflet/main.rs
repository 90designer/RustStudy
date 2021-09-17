// match
// 必须穷举所有的模式

struct guo_jia{
    name:String,
    coin:String,
    price:u8,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    RMB(guo_jia),
}
fn main() {
    let dime = Coin::Dime;
    println!("{}", take_value(dime));

    let gj = guo_jia{
        name:String::from("中国"),
        coin:String::from("RMB"),
        price: 100,
    };
    let rmb = Coin::RMB(gj);
    println!("{}",take_value(rmb));


    let five = Some(5);


    let c = 10;
    let mut strAttr = "";
    match c {
        1 => strAttr = "1",
        2 => strAttr = "2",
        _ => strAttr = "3", // 剩余匹配
    }
    println!("{}", strAttr);


    iflet();
}

fn take_value(coin:Coin)->u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=>{
            println!("多行的写法");
            3
        },
        Coin::Dime => 5,
        Coin::RMB(val)=>{  // 将对应的值，赋值给val
            if val.price > 100 {
                println!("大于100了");
            }else{
                println!("小于等于100");
            }
            val.price
        }
    }
}

// Ooption<T>枚举
fn add_one(x :Option<i32>)->Option<i32>{
    match x {
        None => None,
        Some(val)=>{
            Some(val+1)
        }
    }
}


    // =========== if let ============
fn iflet(){
    let mut v =100;
    if let 100 = v {
        v = 200;
    }else{
        v = 30;
    }

    println!("if let {}", &v);
}