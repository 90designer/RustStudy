pub struct stu {
    pub name:String,
    age:i32, // 私有变量
    pub height:i8
}
impl stu {
    pub fn get_stu(name:&str, height:i8)->stu{
        stu { name: String::from(name), age: 20, height}
    }
}

// rust中枚举变体默认都是公共的，否则定义就没意义了。
pub enum Color{
    red(i8),
    green,
    yellow,
}

pub mod one1;

mod one2{
    fn div(){
        super::fn1();
        super::one1::add();
        crate::one::one1::add();
    }
}

pub fn fn1(){}