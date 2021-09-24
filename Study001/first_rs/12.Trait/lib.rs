use std::fmt::{Debug, Display};

pub trait Actions{
    fn eat (&self)->String;

    // this is an associated function(方法), not a method（函数）
    fn look()->String;

    fn get_name(&self)->String;

    // 内部函数调用
    fn head(&self)->String{
        format!("{}:有头", self.get_name())
    }
    // 默认方法：所有实现这个Trait的数据结构都可以调用
    fn jiao(names:&Vec<&String>)->String{

        let mut fm = String::new();
        for &n in names  {
            fm = fm + "," + &n;
        }
        fm += &":都可以叫";
        fm
    }

    // 函数参数不加&self,就类似一个静态方法，只能使用实现这个trait的数据结构名称::调用
    // 并且不能相互调用
}

pub struct Cat{
    pub name:String,
    pub age:i8
}

// 为struct实现trait方法
impl Actions for Cat {
    // Trait声明的方法默认都是公共的
    fn eat(&self) ->String{
        format!("{}:我可以吃", &self.name)
    }
    fn look() ->String {
        format!("cat:我可以看")
    }
    fn get_name(&self)->String{
        format!("{}",&self.name)
    }
    // Trait没有定义的方法，这里不可以定义
    // fn run(&self)->String{
    //     format!("{}:我可以跑",&self.name)
    // }
}

// 为struct定义方法必须声明是否私有或公共
impl Cat {
    pub fn run(&self)->String{
        format!("{}:我可以跑",&self.name)
    }
}
pub struct Dog{
    pub name:String,
    pub age:i8
}

impl Actions for Dog {
    fn eat(&self) ->String{
        format!("{}:我可以吃", &self.name)
    }
    fn look() ->String {
        format!("Dog:我可以看")
    }
    fn get_name(&self)-> String{
        format!("{}",&self.name)
    }
}

// impl trait用于函数参数
pub fn notify(ac : impl Actions){
    println!("实现这个trait的都可以");
}
// trait bound方式
pub fn notify2<T: Actions>(ac: T){
}
pub fn notify2_1<T: Actions+Display>(ac: T){
}

// 同时定义两个类型(参数是trait类型才可以这么写)
pub fn notify3(ac : impl Actions+Display){
    println!("实现这个trait的都可以");
}

// 简化函数签名：where
pub fn not1 <T: Actions + Display, U: Clone+Debug>(a:T, b:U)->String{
    format!("AAA")
}
// 简化写法，where 可以一行，可多行
pub fn not2 <T, U>(a:T, b:U)->String 
    where 
        T:Actions + Display,
        U:Clone+Debug 
{
    format!("AAA")
}

// trait作为函数返回值
pub fn return_trait() -> impl Actions{
    Dog{
        name:"旺旺".to_string(),
        age:10,
    }
}
// 作为返回值时，返回的类型就算是实现了这个trait的不同类型，也不可以
// pub fn return_trait2(flag:bool) -> impl Actions{
//     if(flag){
//         Dog{   // 返回Dog类型
//             name:"旺旺".to_string(),
//             age:10,
//         }
//     }else{
//         Cat{  // 返回Cat类型 两种类型不同，虽然都实现了Actions
//             name:"喵喵".to_string(),
//             age:11,
//         }
//     }
// }