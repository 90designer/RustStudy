use adder_lib;  // 引入依赖的项目
use std::{fmt::{self, write}, vec};
/**
 *  === 通过例子学习Rust ===
 */
fn main(){
    let num /***语句内的注释* */ = 10i32; //注释数值类型
    println!("输出结果：{}", adder_lib::add_one(num));


    let str0 = "ABCDEFG";
    let str1 = "HIJKLMN";
    // 可以使用位置参数
    println!("`{0}`， this is `{1}` ,that is `{0}`", str0, str1);  //{NUMBER}表示第几个变量，从0开始

    // 可以使用命名参数
    println!("{str2}, 是命名参数的使用", str2 = "哈哈哈");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{:b}",1);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);
    println!("{number:>0wd$}", number=2, wd=6);

    println!("{num1:18}", num1=1);

    println!("{1} {} {} {0}" , 1, 2); // {} 按照正常的循序输出，{NUMBER}按照位置输出

    println!("{:<5}!", "X");
    println!("{:>5}!", "X");
    println!("{:^10}!", "X");
    println!("{:-^10}!", "X");
    println!("{} {1:\">5}!", "X","A");

    // {1:0.0$} 表示打印输出 索引1的参数（3.1415926）, 0表示填充0， .表示小数点 ， 0$表示索引为0的参数。
    println!("Hello is {1:0.0$}", 10, 3.1415926);
    println!("{:?}", iabc(100));
    
    // Debug显示输出 和 Display显示输出

    let c = Complex{real:3.3, imag:7.2};
    println!("Display: {}", &c);
    println!("Debug: {:?}", &c);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

    // 1.2.4 格式化
    println!("1.2.4 格式化: {}", format!("{:o}", 2034));



}

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct iabc(i32);

// 手动实现fmt::Display
struct StructI32 (i32);
impl fmt::Display for StructI32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real:f32,
    imag: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{0} + {1}i", self.real, self.imag)
    }
}


struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}:{}", count,v)?;
        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}
