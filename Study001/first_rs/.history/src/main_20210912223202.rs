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
    println!("{}",s1);
}
// 克隆 == 栈和堆上的数据都复制。
fn string_fn3(){
    let s1 = String::from("s1堆中的数据");
    let s2 = s1.clone(); 
}