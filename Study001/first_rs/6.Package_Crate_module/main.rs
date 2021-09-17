// Package  Crate  Module Path

/*
    Crate 单元包
    生成：binary(二进制文件)或者library（库文件）

    一个Package：
    -- 包含一个Cargo.toml, 描述了如何构建这些Crates
    -- 只能包含0~1个library crate
    -- 可以包含任意个binary crate
    -- 但必须至少包含一个crate(library或者binary)

    src/main.rs:
        -- binary crate的crate root (crate的入口文件)
        -- crate名与package名相同
    src/lib.rs:
        -- package 包含一个library crate
        -- library crate的crate root (crate的入口文件)
        -- crate名与package名相同

    module 模块包:
        -- 定义：mod 名称{} 
        -- 定义私有边界（public（pub）  private(什么都不加)）
        -- mod可嵌套定义
        -- 父级mod无法访问子mod的私有条目
        -- 子mod可以使用父mod所有的条目
        
    Path 路径:
        -- 相对路径：super/self::xxx::xxx::xx
        -- 绝对路径：crate::xxx::xxx::fn()

*/
// =====【use 清理】====
// use std::{cmp::Ordering, io};  
// use std::io::{self , Write};   // 引入io自身和io下的Write
// use std::collections::*;  // 引入所有的条目
fn main(){
    println!("a");
}