
// =======[rust 错误处理 ]======
/**
 *  panic!
 * 
 *  Result枚举
 */
use core::panic;
use std::{fs::File, io::{ErrorKind, Read}, path::Path};

fn main() {
    let file_name = Path::new( "a.txt");
    let display = file_name.display();

    // Result 枚举 Ok , Err变体
    let mut f = File::open(&file_name);
    let mut f = match f {
        Ok(file) =>file,
        Err(error)=> match error.kind(){
            // ErrorKind 枚举 NotFound , PermissionDenied等等
            ErrorKind::NotFound => match File::create(&file_name){
                Ok(fc) => fc,
                Err(e)=> panic!("创建文件失败：{:?}",e),
            },
            ErrorKind::PermissionDenied => panic!("无权限"),

            // 自定义类型：表示其他ErrorKind类型都执行这一步。
            other_error=> panic!("其他错误：{:?}",other_error),  
        }
    };

    let mut s = String::from("测试1/n");
    match f.read_to_string(&mut s){
         Err(why) => panic!("couldn't read {}: {}", display, why),
         Ok(_) => print!("{} contains:\n{}", display, s),
    }

    bibao::fs_fn();


}
//  简化方式1
mod bibao{
    use core::panic;
    use std::{fs::File, io::ErrorKind};

    pub fn fs_fn (){
        let f = File::open("a.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("a,txt").unwrap_or_else(|error| {
                    panic!("创建文件失败:{:?}", error);
                })
            }else{
                panic!("打开文件失败：{:?}",error);
            }
        });

        println!("打开成功:{:?}",f);
    }
}

// 简化方式2 ： ？
mod wenhao {
    use std::{ fs::File, io::{self, Read}};
    pub fn read_from_file() -> Result <String, io::Error> {
        let mut f = File::open("a.txt") ?;
        // let mut f = match f{
        //     Ok(file) => file,
        //     Err(e) => return Err(e),
        // };

        let mut s = String::new();
        f.read_to_string(&mut s)?;

        // let mut s = String::new();
        // match就是这个函数的返回值
        // match f.read_to_string(&mut s){
        //     Ok(_) => Ok(s),
        //     Err(e) => Err(e),
        // }
        Ok(s)
    }

    // 链式调用(更简洁)
    pub fn read_from_file2() -> Result <String, io::Error>{
        let mut s = String::new();
        File::open("a.txt") ?.read_to_string(&mut s)?;
        Ok(s)
    }
}
