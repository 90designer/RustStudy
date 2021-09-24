use std::fmt::Display;

/*
    rust 生命周期
    存在的目的：避免悬垂应用

    借用检查器

    'a 生命周期的标注：
        1. 不会改变引用的生命周期长度
        2. 指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
        3. 描述了多个引用的生命周期的关系。

    单个生命周期的标注本身没有意义。

    函数返回值的生命周期，取用的是参数中生命周期最短的那个。

    每个引用都有生命周期。

    'static 静态生命周期
    所有的字符串字面值都拥有这个生命周期。

    Rust:生命周期省略的三条规则
    第一条规则是，每一个引用参数都会拥有自己的生命周期参数。
    第二条规则是，当只存在一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数。
    第三条规则是，当拥有多个输入生命周期参数，而其中一个是&self或&mut self时，self的生命周期会被赋予给所有的输出生命周期参数
*/

fn main(){
}

//  this function's return type contains a borrowed value,
//  but the signature does not say whether it is borrowed from `x` or `y`
// fn long_comp (x:&str, y:&str) ->&str{
//     if x.len() > y.len(){
//         x
//     }else{
//         y
//     }
// }

// 'a 生命周期的标注：
fn long_comp<'aaa> (x:& 'aaa str, y:&'aaa str) ->&'aaa str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}

struct Stu<'a>{
    name:&'a String,

}


// 泛型 生命周期
// 必须在泛型类型参数之前声明生存周期参数
fn longest_with_display<'a,T>
    (x : &'a str, y:&'a str, title:T)-> &'a str
    where T:Display
{
    println!("函数：{}", title);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}