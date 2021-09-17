mod one;  // 引入本地的mod

//  module 分层树结构：
// mod one 
    // struct stu
    // enum Color
    // mod one1
        // mod one11
        // mod one12
    // mod one2


// use crate::one; // 绝对路径 （one module 本身就是这个文件里的，再引入有点重复）
use one::one1;  // 相对路径
use one::stu;   // 结构体等直接引用
pub use self::one::Color as cl; // 别名 , 同时外部代码也可以使用了【pub use】

// 如果其他文件想引用这个函数，是可以的，但是use引入的方法无法访问，因为是私有的
pub fn eat(){
    let color = cl::red;
}
pub fn eat2(){
    let color = cl::red;
}

pub fn at (){
    one1::add();
    one::fn1();

    let name = "小米";
    let height = 100;
    let mut stu_1 = stu::get_stu(name, height);
    stu_1.name = String::from("哈哈");
    // stu_1.age = 10; // 私有属性无法修改
}
