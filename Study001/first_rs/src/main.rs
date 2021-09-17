
use first_rs::Cat;
use first_rs::Dog;
use first_rs::Actions;

// =======[Rust Trait] =====
/**
 *  需要引入Trait
 */

fn main() {
        let cat = Cat{
            name:"小猫".to_string(),
            age:10,
        };
        println!("{}", cat.eat());

        println!("{}", cat.run());

        let dog = Dog{
            name:"旺旺".into(),
            age:4,
        };
        println!("{}", dog.eat());

        println!("实现trait函数时，没有self参数：{}", Dog::look());

        println!("公共默认方法:{}", Cat::jiao(&vec![&dog.name, &cat.name]) );
        
        println!("trait方法内部调用:{}",cat.head() );
        
}
