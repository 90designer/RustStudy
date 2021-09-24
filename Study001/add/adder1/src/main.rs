
// use pages::rust_1_2_4;  或者使用use引入整个模块
mod pages;

// main
    // pages
        // mod.rs 通过这个固定的文件rust引入内部的其他mod文件
        // mod rust_1_2_4  
   
// 注意：文件本身就是一个mod, 内容如果再出现mod那就是子mod，调用就会加一层
// https://blog.csdn.net/qq_41359051/article/details/108135856 
// 这篇文章讲解了mod 的使用方法。

fn main() {
    let city = pages::rust_1_2_4::City{name:"Dublin",lat: 53.347778, lon: -6.259722};
}