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