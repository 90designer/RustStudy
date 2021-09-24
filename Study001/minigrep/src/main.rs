/***
 * 闭包：
 * 参数类型和返回值类型都可以不用标注，编译器通过使用推断出
 * 
 * Fn Trait:
 *  -- Fn
 *  -- FnMut
 *  -- FnOnce
 * 
 * 闭包可以捕获上下文
 * 
 * 
 */

 use std::collections::HashMap;
fn main() {
    // 匿名函数
    let nm_fn = |a:u32| -> u32{
        a*2
    };

    println!("这是匿名函数的调用：{}",nm_fn(10));



    //=== 使用泛型和Fn Trait实现闭包结果的缓存

    // 1. 定义一个struct 
    // 2. 实现一个初始化方法
    // 3. 实现一个只执行一次的方法
    // 4. 调用
    let mut cache = Cacher::new(nm_fn);
    println!("首次调用查看struct：value={:?}", &cache.value); // None
    println!("执行value方法：{}",cache.value(102)); // 第一次调用初始化完成，
    println!("执行value方法：{}",cache.value(102)); // 第二次调用不再修改
    println!("执行value方法：{}",cache.value(102)); // 第3次调用不再修改

    println!("执行value方法：{}",cache.value(202)); // 第一次调用初始化完成，

    println!("执行value方法：{}",cache.value(1000));// 第一次调用初始化完成，
    println!("执行value方法：{:?}",cache.value);


    // === move
    let base_num = String::from("AAA");
    let flag = String::from("BBB");
    let move_fn = move |x:String| ->bool {
        x == base_num || x == flag
    };
    // let b = base_num;  // base_num 已经被移动到闭包
    // let c = flag;      // flag 已经被移动到闭包
}

struct Cacher<T> 
    where T: Fn(u32)->u32   // 泛型类型的约束
{
    closure:T,
    value:HashMap<u32, u32>,
}

impl<T:Fn(u32)->u32 > Cacher<T> {
    fn new(closure_fn:T)->Cacher<T>{
        let hm: HashMap<u32,u32> = HashMap::new();
        Cacher{
            closure:closure_fn,
            value: hm,
        }
    }
    fn value(&mut self, arg: u32) ->u32{
        match self.value.get(&arg) {
            Some(v)=> *v,
            None => {
                println!("没有该值（{}），首次初始化",&arg);
                let v =  (self.closure)(arg);
                self.value.insert(arg, v);
                v               
            }
        }
    }
}