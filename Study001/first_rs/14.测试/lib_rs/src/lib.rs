

// pub struct Cercle{
//     width:i32,
//     height:i32,
// }

// impl Cercle {
//     pub fn contain(&self , other: &Cercle)->bool{
//         self.width >= other.width && self.height >= other.height
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn can_hold(){
//         let cc1 = Cercle{width:100, height:30,};
//         let cc2 = Cercle{width:122, height:50,};
//         //assert!(cc2.contain(&cc1),"自定义错误信息");
//         assert_eq!(cc1.height,cc2.height,"两个数值不相等");
//     }
//     /**
//         ---- tests::can_hold stdout ----
//         thread 'tests::can_hold' panicked at 'assertion failed: `(left == right)`
//         left: `30`,
//         right: `50`', src\lib.rs:23:9
//         note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//      */

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
//     // #[test]
//     // fn echo (){
//     //     panic!("AAAAAAAA");
//     // }
// }


// pub struct Guess{
//     value:u32
// }

// impl Guess {
//     pub fn new(val:u32) -> Guess{
//         if val< 1  {
//             panic!("Val1 must be between 1 and 100, got {}", val);
//         } else if val > 100  {
//             panic!("Val100 must be between 1 and 100, got {}", val);
//         }
//         Guess{
//             value:val
//         }
//     }
// }
// #[cfg(test)]
// mod tests2{
//     use super::*;

//     #[test]
//     #[should_panic(expected="Val100")] //允许panic，就可以通过( 如果错误panic包含expected的内容，就会通过，否则失败 )
//     fn great_than_100(){
//         Guess::new(0);
//     }
// }

// 测试中使用Result<T,E>
// #[cfg(test)]
// mod tests3{
//     #[test]
//     fn it_works()->Result<(), String>{
//         if 2+2 == 5{
//             Ok(())
//         }else{
//             Err(String::from("两个数不相等o "))
//         }
//     }
// }

// cargo test 默认：
    // 并行执行（默认使用多线程并行运行）
        // 确保测试不会相互依赖
        // 不依赖于某个共享状态（环境、工作目录、环境变量等）
    // 所有测试
    // 捕获所有输出，不显示println!等，测试失败会显示

// cargo test -- --help  查看--参数后还有哪些测试
    // -- --test-threads  测试使用的线程数量: -- --test-threads =1
    // 例如： cargo test -- --show-output (成功通过的测试也打印输出信息)

// pub fn print_and_return_10(a:i32)->i32{
//     println!("I got {}",a);
//     10
// }

// #[cfg(test)]
// mod tests4{
//     use super::*;
//     #[test]
//     fn this_test_fail(){
//         let val = print_and_return_10(1);
//         assert_eq!(6, val);
//     }
//     #[test]
//     fn this_test_pass(){
//         let val = print_and_return_10(4);
//         assert_eq!(10, val);
//     }
    
// }


// 指定测试的方法名测试
// 单个方法测试
    // cargo test fn_name（准确的名字）
    // 如果：有两个方法：fn aa(), fn aa2()  , aa就有可能是模糊匹配
// 多个方法测试
    // cargo test this( 方法名或者模块名中包含this的都测试 )
// 忽略测试：#[ignore]
    // 仅执行ignore标记的方法测试：  cargo test -- --ignored
// #[test]
// #[ignore]
// fn ignore_fn(){
//     assert_eq!(10,10);
// }


// 测试可以使用私有函数

// 单元测试  #[cfg(test)]标注
    // ---- 只有运行cargo test才编译和运行
    // ---- 运行cargo build不会执行。
// 集成测试 不需要标注
// 在src同级目录下，建立tests文件夹，建立要执行测试的测试文件（不需要标注#[cfg(test)]  ）, cargo test执行测试

// cargo test --test tests下的文件名  ： 指定要测试的文件
pub fn jicheng(a:i32)->i32{
    a*2
}

// 帮助函数
// 1. 在tests目录下，建立子文件夹
// 2. 再在子文件夹中建立mod.rs文件
// 3. 使用mod 子文件夹名引入到测试文件中。


