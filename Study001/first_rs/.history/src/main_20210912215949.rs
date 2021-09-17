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
    // ======[Rust变量和可变性]=====
    let a  = 5;
    let b = 5.01;
    let c = true;
    let d = 'a';
    let e = '\u{1f601}';
    let f  = "string"; //  不可变的字符串
    let mut g = "mut string";  // 可变变量
    g = "哈哈哈可变的";

    // =====[Rust 常量]=====
    // 1, 不可以使用mut 常量永远不可变
    // 2，常量使用const，类型必须标注
    // 3，常量可以在任何作用域内声明，包括全局作用域
    // 4, 常量只可以绑定到常量表达式,不可以绑定到函数的调用结果,
    //    也不可以绑定在运行时才能计算出的值
    // 5, 在程序运行期间，常量在其声明的作用域内一直有效。
    // 6，数字可加下划线，增强可读性。
    const MAX_POINTS:u32 = 100_0000; 

    // =====[Rust Shadowing]=====
    // 变量名可以重复定义(名字相同，但是新的变量，如果没有使用let将会报类型错误)
    let x = 1;
    let x = x+ 1;
    let x = x * 6;

    let spaces = "    ";
    let spaces = spaces.len(); // 根据计算机架构 usize

    // ===== [Rust 数据类型] ====
    // Rust标量类型（4种）
    // 1. 整数类型
    // 2. 浮点类型
    // 3. 布尔类型
    // 4. 字符类型 （4个字节）
    let fd = 2.01;
    let fd2:f32 = 100.0;
    let u_int:u8 = 255;
    let char_attr = 'z';
    let char_attr:char = 'v';
    let char_attr:char = '\u{D7ff1}';

    // Rust 复合类型（2种）
    // 可将多个值放在一个类型里
    // 1. 元组（Tuple） 
        // 长度固定
        // 多个类型值
    // 2. 数组
        // 可以多个类型值
        // 每个元素类型必须相同
        // 长度也是固定的

    let tuple1 = (100,-2.1,100000);
    let tuple2:(i8,char, &str) = ( -10, '1',"字符串");
    // 类似js的解构语法
    let (mut t1, t2, t3 ) = tuple2; 
    // 获取元组中元素
    println!("元组：{},{},{}", tuple2.0, tuple2.1, tuple2.2);

    let arr1 = [1,2,3,4];
    // 定义数组类型，[类型; 长度]
    let arr2:[char;3] = ['a','b','c'];
    let arr3 = ['C';3] ; // 标识['C','C','C']
    println!("{}",arr2[0]);
    // ===== [Rust 函数] ====
    //  大部分函数都使用最后一个表达式作为返回值（没有分号）
    //  函数参数必须指定类型
    another_function("测试函数");

    let yy = {
        let xx = 100;
        xx+100    // 没有分号，标识整个语句是返回值
    };
    println!("块：{}",yy);
    let fn1_num = fn1();
    println!("函数返回值：{}",fn1_num);
    
    // ===== [Rust if] ====
    let if_num1 = 100;
    if if_num1 < 1000 {
        println!("小于");
    }else if if_num1 > 10 {
        println!("小于");
    }
    else{
        println!("大于");
    }

    // if表达式作为值
    // 【rust要求编译时必须确定变量的类型】
    let bool_info = false;
    let if_num2 = if bool_info { 100} else{ 10 }; // 类型必须相同
    println!("if表达式作为值：{}",if_num2);

    // =======[Rust 循环 ]========
    // loop  while  for
    let mut n1 = 10;
    let loop_result = loop{
        if n1 >= 0{
            println!("n = {}", n1);
        }else{
            println!("循环结束.....");
            break n1;  // 循环结束，返回结果
        }
        n1 = n1 -1;
    };
    println!("loop循环：{}",loop_result);
    
    
    let mut n2 = 10;
    while n2 != 0{
        println!("n2 = {}", n2);
        n2 = n2 - 1;
    }
    println!("while循环: {}", n2);


    let arr2 = [10,20,30,40,50];
    let mut index = 0;
    while index < arr2.len() {
        println!("index = {}:{}", index, arr2[index]);
        index = index +1;
    }




    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);
    println!("{}", g);
    println!("{}", MAX_POINTS);
    println!("{}", x);
    println!("{}", u_int);
    println!("{}", char_attr);
}

fn another_function (x:&str){
    println!("another_function:{}",x)
}
// 定义返回值类型
fn fn1()->i64{
    5
}
