
// =======[rust struct结构体]===
struct User{
    user_name :String,
    email:String,
    age:u64,
    active: bool,    
}

// =======[rust Tuple Struct 元组结构体]===
struct Animal(String, u64, char);
struct Color();

fn main() {
    // 所有字段要么不可变，要么可变
    let user_name = String::from("你好啊");
    let mut user1 = User{
        email:String::from("email@11.com"),
        user_name,  // 简写
        age:10,
        active:false,
    };

    let user2 = User{
        age:20,
        active:true,
        ..user1  // 语法糖：其他字段的值都与user1的相同。
    };
    user1.email = String::from("e1@qq.com");
    println!("user1.email = {}",user1.email);


    let cat = Animal(String::from("猫"), 10, 'M');
    let cat1 = cat.1; // 元组的索引方式

    // =====[rust Unit-Link Struct 没有任何字段]=====
    // 与() 单元类型 类似


    // 例子：
    get_mianji();
}

// struct例子： 计算长方形面积
#[derive(Debug)]
struct Rec{
    width:u32,
    height:u32,
}
// 结构体定义方法 
//  必须使用impl 定义
//  一个Struct可以定义多个impl
impl Rec {
    fn area3(&self)-> u32{
        self.height * self.width
    }
    fn can_hold(&self, other:&Rec) ->bool{
        self.width > other.width && self.height > other.height
    }
}
impl Rec {
    // 关联函数 （类似静态函数）
    fn square(size:u32)-> Rec{
        Rec{
            width:size,
            height:size
        }
    }
}
fn get_mianji(){
    let rec = (30,5);
    println!("tuple 长方形面积：{}", area(rec));

    let rec2 = Rec{width:30, height:5};
    println!("struct 长方形面积：{}", area2(&rec2));
    // 打印自定义类型， {:?}打印输出结构，{:#?}格式化打印输出结构（必须在要打印的自定义类型之前添加：#[derive(Debug)]）
    println!("{:#?}", rec2); 
    println!("{}", rec2.area3()); 

    let rec3 = Rec{width:100,height:30};
    let rec4 = Rec{width:40,height:20};
    println!("是否可以包含？：{}", rec3.can_hold(&rec4));

    println!("{:#?}", Rec::square(100));

}
// 元组方式
fn area(dim:(u32,u32)) ->u32{
    dim.0 * dim.1
}
// struct方式
fn area2(rec :&Rec) ->u32{
    rec.width * rec.height
}
