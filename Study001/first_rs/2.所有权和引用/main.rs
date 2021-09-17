use std::string;

// Rust 核心特性：所有权

// Stack栈内存
    // 后进先出（LIFO）
    // 添加数据 == 压入栈
    // 移除数据 == 弹出栈

    // 所有存储在Stack上的数据必须有已知的固定的大小，未知或者运行时改变的数据，存储在heap上。

// Heap 堆内存

// 所有权规则

fn main(){
    string_fn();
    get_yinyong();
    get_yinyong3();
    get_first_word_index();
    str_slice();
}
// 变量在离开自己所在的作用域之前会调用drop()函数，销毁。
fn string_fn(){
    let mut s = String::from("Hello");
    s.push_str(" world");
    println!("{}",s)
}
// 移动 == 浅拷贝+销毁
fn string_fn2(){
    let s1 = String::from("s1堆中的数据");
    let s2 = s1; // 此时s1已经移动到s2,s1不存在了，防止出现两次释放内存。
    // println!("{}",s1);
}
// 克隆 == 栈和堆上的数据都复制。
fn string_fn3(){
    let s1 = String::from("s1堆中的数据");
    let s2 = s1.clone(); 
}

// 所有权与函数
fn suoyouquan_func(){
    let s = String::from("Hello");
    // 传入函数等同于赋值给变量，s已经被移动到函数中，s无效（函数结束时，无需对s进行内存释放操作）
    take_ownership(s);
    // println!("{}",s); // 引用了被移动的数据，s已经无效
    let x = 5;
    make_copy(x);
    println!("x: {}",x);
}

fn take_ownership(some_thing:String){
    println!("{}",some_thing);
}
fn make_copy(some_thing:i32){
    println!("{}",some_thing);
}
// 引用，并没有取得所有权（变量不会失效）
fn get_yinyong(){
    let s1 = String::from("你好"); // 中文占用3个字符长度
    let len = get_length(&s1); // 引用
    println!("{},{}",len,s1);

    let mut s2 = String::from("你好啊");
    let len = get_length2(&mut s2);
    println!("len={},s2={}",len,s2);
}

// 引用的变量默认是不可变的
fn get_length(some:&String)->usize{
    // some.push_str("修改引用的变量");  
    some.len()
}
// 引用的可修改的变量是可以修改的
fn get_length2(some:&mut String)->usize{
    some.push_str("修改引用的变量");
    some.len()
}
// 引用的可修改的变量，同一作用域下只能有一次（防止数据竞争）
fn get_yinyong2(){
    let mut s1 = String::from("哈哈哈");
    let s2 = &mut s1;
    // let s3 = &mut s1;
    // println!("{},{}",s2 ,s3);
}
fn get_yinyong3(){
    let mut s = String::from("AAA");
    {
        let s1 = &mut s;
        println!("get_yinyong3:{}", s1.len());
    }
    let s2 = &mut s;
    s2.push_str("创建的s2也是可变的，虽然没有指定");
    println!("get_yinyong3:{}",s2);
}

// 不可以同时存在可变引用和不可变引用
fn get_yinyong4(){
    let mut s = String::from("可变变量");
    let s1 = &s; // 不可变的引用
    let s2 = &s; // 不可变的引用2
    // let s3 = &mut s; // 可变引用；
    // let s4 = &mut s; // 可变引用只能使用一次；
    println!("{},{}",s1,s2);
}

// 悬空指针（指向已经被释放的变量）rust在编译时就会避免
// fn get_yinyong5()->&String{
//     let s = String::from("悬空指针");
//     &s  // s在函数执行完成就已经被释放，指针指向的是个空；
// }

// =======引用的规则（同一个变量）====
 // 1. 只能有一个可变的引用
 // 2. 可以有任意数量不可变的引用
 // 3. 引用必须一直有效 
 //（不可以多个可变引用， 可变和不可变不可以同时使用，引用不能悬空）

 
//  =======[Rust 切片slice]========
fn get_first_word_index(){
    let mut s = String::from("0123 567");
    // println!("索引：{}",s);
    let index = first_word(&s);
    let index2 = first_word_slice(&s[..]);
    // 如果此处我们清空了字符串
    // s.clear();
    // 那么索引就会失效，造成程序bug，rust通过字符串切片解决。
    println!("索引：{},{}",index,index2);
}
fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i   // 要return 
        }
    }
    s.len()
}
fn first_word_slice(s:&str)->&str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i];
        }
    }
    &s[..]
}

// ======== 字符串切片 =======
// 字符串切片：指向字符串以部分内容的切片
fn str_slice(){
    // 字符串切片的范围索引必须是有效的UTF-8的字符边界内；
    // 如果尝试从一个多字节的字符中创建字符串切片，程序报错并退出
    // let s = String::from("哈哈哈哈哈哈哈哈");
    let s = String::from("0123456789");
    let s1 = &s[0..5];
    let s2 = &s[..5];
    let s3 = &s[5..s.len()];
    let s4 = &s[5..];
    let s5 = &s[..];
    println!("字符串切片：{},{},{},{},{}",s1,s2,s3,s4,s5);
}